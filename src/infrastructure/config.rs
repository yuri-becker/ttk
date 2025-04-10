pub struct Config {
    pub bearer: String,
    pub user_agent: String,
}

impl Config {
    pub fn from_dotenv() -> Self {
        Self {
            bearer: dotenv::var("THALIA_BEARER")
                .expect("Please define THALIA_BEARER in the .env.local"),
            user_agent: dotenv::var("THALIA_USER_AGENT")
                .expect("Please define THALIA_USER_AGENT in the .env.local"),
        }
    }
}
