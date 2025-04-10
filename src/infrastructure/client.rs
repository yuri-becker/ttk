use crate::infrastructure::config::Config;
use header::{HeaderMap, HeaderValue};
use reqwest::header;
use std::ops::Deref;

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("X-Clientname", HeaderValue::from_static("crosschannelapp"));
        default_headers.insert(
            "User-Agent",
            HeaderValue::from_str(&config.user_agent).unwrap(),
        );
        default_headers.insert(
            "Authorization",
            HeaderValue::from_str(("Bearer ".to_owned() + &config.bearer).as_str()).unwrap(),
        );
        Self {
            client: reqwest::blocking::Client::builder()
                .default_headers(default_headers)
                .build()
                .expect("Could not initialize HTTP client"),
        }
    }
}

impl Deref for Client {
    type Target = reqwest::blocking::Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
