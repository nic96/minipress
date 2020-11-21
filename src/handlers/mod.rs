use crate::handlers::github_oauth2::github_oauth2_config;
use actix_web::web;

mod github_oauth2;
mod post_handlers;
mod user_handlers;

pub use github_oauth2::GithubUserInfo;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .configure(github_oauth2_config)
            .configure(user_handlers::init)
            .configure(post_handlers::init),
    );
}
