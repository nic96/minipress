use oauth2::basic::BasicClient;

pub struct GithubOauth2State {
    pub oauth: BasicClient,
    pub api_base_url: String,
}
