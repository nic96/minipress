use crate::database::DbPool;
use crate::models::{User, UserRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::types::Uuid;

// TODO setup proper middleware/route protection for each of the handlers

// TODO pagination
#[get("/users")]
async fn find_all(db_pool: web::Data<DbPool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::BadRequest().body("Error trying to read all users from database"),
    }
}

#[get("/user/{uuid}")]
async fn find(uuid: web::Path<String>, db_pool: web::Data<DbPool>) -> impl Responder {
    let uuid_;
    match Uuid::parse_str(uuid.as_str()) {
        Ok(u) => uuid_ = u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid User ID"),
    }
    let result = User::find_by_id(uuid_, db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

#[post("/user")]
async fn create(user: web::Json<UserRequest>, db_pool: web::Data<DbPool>) -> impl Responder {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to create new user"),
    }
}

#[put("/user/{uuid}")]
async fn update(
    uuid: web::Path<String>,
    user: web::Json<UserRequest>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let uuid_;
    match Uuid::parse_str(uuid.as_str()) {
        Ok(u) => uuid_ = u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid User ID"),
    }
    let result = User::update(uuid_, user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

#[delete("/user/{uuid}")]
async fn delete(uuid: web::Path<String>, db_pool: web::Data<DbPool>) -> impl Responder {
    let uuid_;
    match Uuid::parse_str(uuid.as_str()) {
        Ok(u) => uuid_ = u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid User ID"),
    }
    let result = User::delete(uuid_, db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("User not found")
            }
        }
        _ => HttpResponse::BadRequest().body("User not found"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
