const BASE_URL: &str = "https://api.discogs.com";

#[derive(Debug)]
pub struct DiscogsClient {
    pub api_endpoint: String,
    pub user_token: String,
    pub username: String,
    pub user_agent: String,
    pub http_client: reqwest::Client,
}

impl DiscogsClient {
    pub fn new(user_token: &str, user_agent: &str, username: &str) -> Self {
        DiscogsClient {
            api_endpoint: BASE_URL.to_string(),
            user_token: user_token.to_string(),
            username: username.to_string(),
            user_agent: user_agent.to_string(),
            http_client: reqwest::Client::new(),
        }
    }
    pub fn custome_endpoint(&mut self, endpoint: &str) {
        self.api_endpoint = endpoint.to_string()
    }
}
