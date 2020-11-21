use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{get, HttpResponse};

#[get("/logout")]
pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found()
        .header(header::LOCATION, "/".to_string())
        .finish()
}
