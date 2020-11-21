use crate::handlers::github_oauth2::GithubOauth2State;
use actix_web::http::header;
use actix_web::{get, web, HttpResponse};
use oauth2::{CsrfToken, PkceCodeChallenge};

#[get("/login")]
pub fn login(data: web::Data<GithubOauth2State>) -> HttpResponse {
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        //.add_scope(Scope::new("user".to_string()))
        //.add_scope(Scope::new("repo".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Found()
        .header(header::LOCATION, auth_url.to_string())
        .finish()
}
