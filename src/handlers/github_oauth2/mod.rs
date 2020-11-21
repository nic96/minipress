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
    let oauth_server =
        dotenv::var("GITHUB_OAUTH2_SERVER").expect("Failed to get the GITHUB_OAUTH2_SERVER .env");
    let auth_url = AuthUrl::new(format!("https://{}/oauth/authorize", oauth_server))
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(format!("https://{}/oauth/access_token", oauth_server))
        .expect("Invalid token endpoint URL");
    let api_base_url =
        dotenv::var("GITHUB_API_URL").expect("Failed to get the GITHUB_API_URL .env variable");

    // Set up the config for the OAuth2 process.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(
        RedirectUrl::new(format!(
            "https://{}/github_oauth2/auth", // TODO store the callback url in the .env
            dotenv::var("APP_URL").unwrap()
        ))
        .expect("Invalid redirect URL"),
    );

    cfg.data(GithubOauth2State {
        oauth: client,
        api_base_url,
    });
    cfg.service(
        web::scope("/github_oauth2")
            .service(login)
            .service(logout)
            .service(auth),
    );
}
