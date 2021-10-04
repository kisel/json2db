use std::{env, error::Error};
use anyhow::{Context, Result};

pub struct Config {
    pub postgres_url: String,
    pub key: String,
    pub api_url: String,
}

fn rdenv(varname: &str) -> Result<String> {
    Ok(env::var(varname).with_context(|| format!("Failed to read env {}", varname))?)
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                postgres_url: rdenv("DATABASE_URL")?,
                key: rdenv("KEY")?,
                api_url: rdenv("API_URL")?,
            }
        )
    }
}
