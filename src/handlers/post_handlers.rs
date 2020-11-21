use crate::database::DbPool;
use crate::models::user::Role;
use crate::models::{Post, PostRequest, User};
use actix_identity::Identity;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::types::Uuid;

// TODO setup proper middleware/route protection for each of the handlers

// TODO pagination
#[get("/posts")]
async fn find_all(db_pool: web::Data<DbPool>) -> impl Responder {
    let result = Post::find_all(db_pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::BadRequest().body("Error trying to read all posts from database"),
    }
}

#[get("/post/{uuid}")]
async fn find(uuid: web::Path<String>, db_pool: web::Data<DbPool>) -> impl Responder {
    let uuid_;
    match Uuid::parse_str(uuid.as_str()) {
        Ok(u) => uuid_ = u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Post ID"),
    }
    let result = Post::find_by_id(uuid_, db_pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Post not found"),
    }
}

#[post("/post")]
async fn create(
    post: web::Json<PostRequest>,
    db_pool: web::Data<DbPool>,
    id: Identity,
) -> impl Responder {
    let identity = match id.identity() {
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
        Some(i) => i,
    };

    let logged_user = match serde_json::from_str::<User>(&*identity) {
        Ok(u) => {
            if u.role != Role::Admin || u.role != Role::SuperAdmin || u.role != Role::Author {
                return HttpResponse::Unauthorized().body("Unauthorized");
            };
            u
        }
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    let result = Post::create(post.into_inner(), db_pool.get_ref(), logged_user).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Error trying to create new post"),
    }
}

#[put("/post/{uuid}")]
async fn update(
    uuid: web::Path<String>,
    post: web::Json<PostRequest>,
    db_pool: web::Data<DbPool>,
) -> impl Responder {
    let uuid_;
    match Uuid::parse_str(uuid.as_str()) {
        Ok(u) => uuid_ = u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Post ID"),
    }
    let result = Post::update(uuid_, post.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        _ => HttpResponse::BadRequest().body("Post not found"),
    }
}

#[delete("/post/{uuid}")]
async fn delete(uuid: web::Path<String>, db_pool: web::Data<DbPool>) -> impl Responder {
    let uuid_;
    match Uuid::parse_str(uuid.as_str()) {
        Ok(u) => uuid_ = u,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Post ID"),
    }
    let result = Post::delete(uuid_, db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("Post not found")
            }
        }
        _ => HttpResponse::BadRequest().body("Post not found"),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
