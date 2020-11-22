mod auth_handler;
mod login_handler;
mod logout_handler;
pub mod state;
use actix_web::web;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use auth_handler::auth;
pub use auth_handler::GithubUserInfo;
use login_handler::login;
use logout_handler::logout;
pub use state::GithubOauth2State;

pub fn github_oauth2_config(cfg: &mut web::ServiceConfig) {
    let github_client_id = ClientId::new(
        dotenv::var("GITHUB_CLIENT_ID").expect("Failed to get the GITHUB_CLIENT_ID .env variable."),
    );
    let github_client_secret = ClientSecret::new(
        dotenv::var("GITHUB_CLIENT_SECRET")
            .expect("Failed to get the GITHUB_CLIENT_SECRET .env variable."),
    );
    let auth_url =
        dotenv::var("GITHUB_AUTH_URL").expect("Failed to get the GITHUB_AUTH_URL .env variable");
    let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");
    let token_url =
        dotenv::var("GITHUB_TOKEN_URL").expect("Failed to get the GITHUB_TOKEN_URL .env variable");
    let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");
    let api_base_url =
        dotenv::var("GITHUB_API_URL").expect("Failed to get the GITHUB_API_URL .env variable");

    // Basic validation of callback url. Url needs to start with a slash and have at least one character after.
    let callback_url = dotenv::var("GITHUB_CALLBACK_URL").unwrap();
    if !callback_url.starts_with('/') {
        panic!("Invalid callback url. Callback url needs to start with a /");
    }
    if callback_url.len() <= 2 {
        panic!("Callback url not valid");
    }

    // Set up the config for the OAuth2 process.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(
        RedirectUrl::new(format!(
            "https://{}{}",
            dotenv::var("APP_URL").unwrap(),
            callback_url,
        ))
        .expect("Invalid redirect URL"),
    );

    cfg.data(GithubOauth2State {
        oauth: client,
        api_base_url,
    });
    cfg.route(callback_url.as_str(), web::get().to(auth));
    cfg.service(web::scope("/github_oauth2").service(login).service(logout));
}
