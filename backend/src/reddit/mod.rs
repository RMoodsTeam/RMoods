use axum::http::HeaderValue;
use log::warn;
use project_root::get_project_root;
use reqwest::{Request, Response};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap, fs::File, io::BufReader, path::PathBuf, thread::panicking,
    time::SystemTime,
};

use crate::REQWEST_CLIENT;

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RedditAccessToken {
    #[serde(rename = "access_token")]
    token: String,
    expires_in: u64,
    #[serde(default = "get_sys_time_in_secs")]
    created_at: u64,
}

impl RedditAccessToken {
    pub fn is_expired(&self) -> bool {
        self.created_at + self.expires_in >= get_sys_time_in_secs()
    }
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RedditApp {
    pub client_id: String,
    pub client_secret: String,
}

impl RedditApp {
    pub async fn fetch_access_token(&self) -> anyhow::Result<RedditAccessToken> {
        let req = REQWEST_CLIENT
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.as_str()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body("grant_type=client_credentials")
            .build()?;

        let res = REQWEST_CLIENT
            .execute(req)
            .await?
            .json::<RedditAccessToken>()
            .await?;
        Ok(res)
    }
}

#[derive(Debug)]
pub struct RedditConnection {
    pub clients: HashMap<RedditApp, Option<RedditAccessToken>>,
    access_token: Option<RedditAccessToken>,
}

impl RedditConnection {
    const API_URL: &'static str = "https://oauth.reddit.com";

    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
    pub fn new() -> RedditConnection {
        let root = get_project_root().expect("Should be able to determine project root");
        let path = PathBuf::from(root).join(".reddit-credentials.json");
        let file = File::open(path).expect("credentials should be present at backend root");
        let reader = BufReader::new(file);
        let json: Value =
            serde_json::from_reader(reader).expect("credentials should be valid JSON");

        let clients: HashMap<RedditApp, Option<RedditAccessToken>> = json
            .as_array()
            .expect("credentials file should only contain an array")
            .iter()
            .map(|i: &Value| {
                (
                    serde_json::from_value::<RedditApp>(i.clone()).expect("Parsable client data"),
                    None,
                )
            })
            .collect();

        assert!(!clients.is_empty());

        RedditConnection {
            clients,
            access_token: None,
        }
    }

    /// Determine which token (from which app) will be used for the next request
    fn current_app(&self) -> &RedditApp {
        self.clients.keys().next().expect("There is one client")
    }

    pub async fn execute(&mut self, mut req: Request) -> anyhow::Result<Response> {
        // If the current token is fine, use the current one. Otherwise fetch a new one and set it as the current
        let token = match &self.access_token {
            Some(t) if !t.is_expired() => t.clone(),
            _ => {
                warn!("Token expired, refetching...");
                let t = self.current_app().fetch_access_token().await.unwrap();
                self.access_token = Some(t.clone());
                t
            }
        };
        if token.is_expired() {
            todo!("Handle expired tokens");
        }

        // Authorize the request
        req.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&token.token).expect("Valid token to header conversion"),
        );

        Ok(REQWEST_CLIENT.execute(req).await?)
    }
}
