use std::{fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RedditClient {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug)]
pub struct RedditConnection {
    pub clients: Vec<RedditClient>,
}

impl RedditConnection {
    /// Read credentials and create a collection of client authentication data
    /// from .reddit-credentials.json in backend root (src/backend/)
    pub fn new() -> RedditConnection {
        let file = File::open(".reddit-credentials.json")
            .expect("Reddit credentials file should be available at project's root");
        let reader = BufReader::new(file);
        let json: Value =
            serde_json::from_reader(reader).expect("credentials should be valid JSON");

        let clients: Vec<RedditClient> = json
            .as_array()
            .expect("credentials file should only contain an array")
            .iter()
            .map(|i: &Value| {
                serde_json::from_value::<RedditClient>(i.clone()).expect("Parsable client data")
            })
            .collect();

        RedditConnection { clients }
    }
}
