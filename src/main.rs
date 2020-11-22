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
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlebars::Handlebars;
use log::LevelFilter;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use simple_logger::SimpleLogger;
use std::str::FromStr;
use time::Duration;

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
            // middlewares
            .wrap(middleware::error_handlers())
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(true))
            .wrap(Compress::default())
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
            // data
            .data(db_pool.clone())
            .app_data(handlebars_ref.clone())
            // services
            .service(
                fs::Files::new("/static", "static")
                    //.use_etag(true)
                    //.use_last_modified(true),
            )
            .configure(init)
    })
    .keep_alive(0)
    .bind_openssl(dotenv::var("APP_URL").unwrap(), builder)?
    .run()
    .await
}
