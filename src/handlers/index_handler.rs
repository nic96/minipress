use crate::models::user::ToUser;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
fn index(id: Identity, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let logged_user = id.user();
    let user_name = match &logged_user {
        Some(u) => u.username.clone(),
        None => "".to_string(),
    };

    let data = json!({
        "name": "Handlebars",
        "user_name": user_name,
        "user": &logged_user,
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
