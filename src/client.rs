use crate::api::pulls::Pulls;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client,
};

pub struct GithubClient {
    pub client: Client,
}

fn build_auth(token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let mut auth_header = HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();

    auth_header.set_sensitive(true);
    headers.insert(AUTHORIZATION, auth_header);

    headers
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::builder()
                .default_headers(build_auth(token))
                .user_agent("dev_analyzer")
                .build()
                .unwrap(),
        }
    }

    pub fn pulls(&self) -> Pulls {
        Pulls::new(self)
    }
}
