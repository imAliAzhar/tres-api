use dotenv::dotenv;

use crate::prelude::*;
use std::sync::OnceLock;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Config {
    pub DATABASE_URL: String,

    pub AIRTABLE_API_KEY: String,
    pub AIRTABLE_BASE_URL: String,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(Config::load_from_env)
}

impl Config {
    fn load_from_env() -> Self {
        dotenv().ok();

        Config {
            DATABASE_URL: env_var("DATABASE_URL"),
            AIRTABLE_API_KEY: env_var("AIRTABLE_API_KEY"),
            AIRTABLE_BASE_URL: env_var("AIRTABLE_BASE_URL"),
        }
    }
}

fn env_var(key: &str) -> String {
    std::env::var(key)
        .map_err(Error::EnvError)
        .unwrap_or_else(|_| format!("ERROR: Env Variable {key} is missing"))
}
