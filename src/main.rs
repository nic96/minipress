mod database;
mod handlers;
mod middleware;
pub mod models;
mod template_helpers;

#[macro_use]
extern crate serde_derive;
extern crate dotenv;

use crate::database::setup_database_pool;
use crate::handlers::init;
use crate::models::User;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use handlebars::Handlebars;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_json::json;
use simple_logger::SimpleLogger;
use std::str::FromStr;
use time::Duration;

#[get("/")]
fn index(id: Identity, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let logged_user = match id.identity() {
        None => None,
        Some(identity) => match serde_json::from_str::<User>(&*identity) {
            Ok(u) => Some(u),
            Err(_) => None,
        },
    };
    let user_name = match logged_user {
        Some(u) => u.username,
        None => "".to_string(),
    };

    let data = json!({
        "name": "Handlebars",
        "user_name": user_name,
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    match SimpleLogger::new()
        .with_level(
            LevelFilter::from_str(
                dotenv::var("LOG_LEVEL")
                    .unwrap_or_else(|_| "INFO".to_string())
                    .as_str(),
            )
            .unwrap(),
        )
        .init()
    {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to setup logger: {}", e),
    }

    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            dotenv::var("SSL_PRIVATE_KEY").expect("Failed to get private key from .env"),
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_certificate_chain_file(
            dotenv::var("SSL_CERTIFICATE_CHAIN")
                .expect("Failed to get ssl certificate chain from .env"),
        )
        .unwrap();

    let db_pool = setup_database_pool().await;

    let mut handlebars = Handlebars::new();
    template_helpers::register_helpers(&mut handlebars);
    // in the future could probably try dynamic template directories to make things more customizable
    // maybe store a temple directory path in the database.
    handlebars
        .register_templates_directory(".hbs", "resources/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::error_handlers())
            .wrap(Logger::default())
            .data(db_pool.clone())
            .app_data(handlebars_ref.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(dotenv::var("SECRET_KEY").unwrap().as_ref())
                    .name("auth")
                    .path("/")
                    .domain(
                        dotenv::var("APP_DOMAIN")
                            .unwrap_or_else(|_| "localhost".into())
                            .as_str(),
                    )
                    .max_age_time(Duration::days(1))
                    .secure(false), // this can only be true if you have https
            ))
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            .service(index)
            .configure(init)
    })
    .keep_alive(75)
    .bind_openssl(dotenv::var("APP_URL").unwrap(), builder)?
    .run()
    .await
}
