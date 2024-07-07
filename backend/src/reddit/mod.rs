use std::{collections::HashMap, fs::File, io::BufReader, time::SystemTime};
use serde::Deserialize;
use serde_json::Value;

use crate::REQWEST_CLIENT;

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct RedditAccessToken {
    #[serde(rename = "access_token")]
    pub token: String,
    pub expires_in: u64,
    #[serde(default = "get_sys_time_in_secs")]
    pub created_at: u64
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RedditClient {
    pub client_id: String,
    pub client_secret: String,
}

impl RedditClient {
    pub async fn fetch_access_token(&self) -> anyhow::Result<RedditAccessToken> {
        let req = REQWEST_CLIENT
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.as_str()))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "application/json")
            .body("grant_type=client_credentials")
            .build()?;

        let res = REQWEST_CLIENT.execute(req).await?.json::<RedditAccessToken>().await?;
        Ok(res)
    }
}

#[derive(Debug)]
pub struct RedditConnection {
    pub clients: HashMap<RedditClient, Option<RedditAccessToken>>
}

impl RedditConnection {
    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
    pub fn new() -> RedditConnection {
        let file = File::open(".reddit-credentials.json")
            .expect(".reddit-credentials.json should be present at backend root");
        let reader = BufReader::new(file);
        let json: Value =
            serde_json::from_reader(reader).expect("credentials should be valid JSON");

        let clients: HashMap<RedditClient, Option<RedditAccessToken>> = json
            .as_array()
            .expect("credentials file should only contain an array")
            .iter()
            .map(|i: &Value| {
                (serde_json::from_value::<RedditClient>(i.clone()).expect("Parsable client data"), None)
            })
            .collect();

        RedditConnection { clients }
    }
}
