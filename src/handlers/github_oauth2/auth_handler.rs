use actix_identity::Identity;
use actix_web::{dev::Payload, get, web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::{err, ok, Ready};
use oauth2::http::{HeaderMap, HeaderValue, Method};
use oauth2::reqwest::http_client;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, TokenResponse};
use url::Url;

use crate::database::DbPool;
use crate::handlers::github_oauth2::GithubOauth2State;
use crate::models::{user::Role, User, UserRequest};
use actix_web::http::header;
use oauth2::http::header::AUTHORIZATION;
use std::time::SystemTime;
use time::PrimitiveDateTime;

pub type LoggedUser = User;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(HttpResponse::Unauthorized().body("Unauthorized").into())
    }
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GithubUserInfo {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: Option<String>,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: u64,
    pub following: u32,
    #[serde(with = "iso_8601_date_format")]
    pub created_at: PrimitiveDateTime,
    #[serde(with = "iso_8601_date_format")]
    pub updated_at: PrimitiveDateTime,
}

#[get("/auth")]
pub async fn auth(
    id: Identity,
    data: web::Data<GithubOauth2State>,
    params: web::Query<AuthRequest>,
    db_pool: web::Data<DbPool>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let _state = CsrfToken::new(params.state.clone());

    // Exchange the code with a token.
    let token = &data
        .oauth
        .exchange_code(code)
        .request(http_client)
        .expect("exchange_code failed");

    let user_info = read_user(&data.api_base_url, token.access_token());

    let user;
    match User::find_by_github_id(user_info.id as i64, db_pool.get_ref()).await {
        Ok(u) => user = u,
        Err(_) => {
            match User::create(
                UserRequest {
                    username: user_info.login,
                    email: user_info.email,
                    password: None,
                    name: user_info.name,
                    avatar_url: user_info.avatar_url,
                    gravatar_id: Some(user_info.gravatar_id),
                    github_id: Some(user_info.id as i64),
                    github_token: Some(token.access_token().secret().to_string()),
                    role: Role::Subscriber,
                    created_at: PrimitiveDateTime::from(SystemTime::now()),
                    updated_at: PrimitiveDateTime::from(SystemTime::now()),
                },
                db_pool.get_ref(),
            )
            .await
            {
                Ok(u) => user = u,
                Err(_) => return HttpResponse::BadRequest().body("Failed to find or create user"),
            }
        }
    }

    id.remember(serde_json::to_string(&user).unwrap());

    // TODO: redirect to previous url
    HttpResponse::Found()
        .header(header::LOCATION, "/".to_string())
        .finish()
}

fn read_user(api_base_url: &str, access_token: &AccessToken) -> GithubUserInfo {
    let url = Url::parse(format!("{}/user", api_base_url,).as_str()).unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(format!("token {}", access_token.secret()).as_str()).unwrap(),
    );
    let resp = http_client(oauth2::HttpRequest {
        url,
        method: Method::GET,
        headers,
        body: Vec::new(),
    })
    .expect("Request failed");
    serde_json::from_slice(&resp.body).unwrap()
}

mod iso_8601_date_format {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::PrimitiveDateTime;

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT);
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(s, FORMAT).map_err(serde::de::Error::custom)
    }
}
