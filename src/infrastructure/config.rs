use anyhow::{anyhow, Result};

pub struct Config {
    pub bearer: String,
    pub user_agent: String,
}

impl Config {
    pub fn from_dotenv() -> Result<Self> {
        Ok(Self {
            bearer: dotenv::var("THALIA_BEARER")
                .map_err(|_| anyhow!("Please define THALIA_BEARER in the .env.local"))?,
            user_agent: dotenv::var("THALIA_USER_AGENT")
                .map_err(|_| anyhow!("Please define THALIA_USER_AGENT in the .env.local"))?,
        })
    }
}
