use std::env;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn read() -> Result<Config> {
        Ok(Config {
            database_url: env::var("DATABASE_URL").map_err(|_| Error::ConfigError {
                setting: "DATABASE_URL".to_string(),
            })?,
        })
    }
}
