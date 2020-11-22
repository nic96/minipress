use crate::handlers::github_oauth2::github_oauth2_config;
use actix_web::web;

mod favicon_handlers;
mod github_oauth2;
pub mod index_handler;
mod post_handlers;
mod user_handlers;

pub use github_oauth2::GithubUserInfo;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(index_handler::init)
            .configure(user_handlers::init)
            .configure(post_handlers::init)
            .configure(favicon_handlers::init)
            .configure(github_oauth2_config),
    );
}
