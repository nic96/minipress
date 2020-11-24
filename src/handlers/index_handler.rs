use crate::models::user::ToUser;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
fn index(id: Identity, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let logged_user = id.user();

    let data = json!({
        "user": &logged_user,
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
