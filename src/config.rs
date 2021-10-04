use std::{env, error::Error};
use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "json2db", about="Scrapes api url and pushes it to remote postgres db")]

pub struct Config {

    /// Target database URL
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// Database record group key
    #[structopt(long, env = "KEY")]
    pub key: String,

    /// API url to scrab
    #[structopt(long, env = "API_URL")]
    pub api_url: String,

    /// Scrape interval in seconds
    #[structopt(short, long, env = "INTERVAL", default_value = "60")]
    pub interval_sec: u32,
}

// Code below is not used anymore. will be removed later

fn rdenv(varname: &str) -> Result<String> {
    Ok(env::var(varname).with_context(|| format!("Failed to read env {}", varname))?)
}

impl Config {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                database_url: rdenv("DATABASE_URL")?,
                key: rdenv("KEY")?,
                api_url: rdenv("API_URL")?,
                interval_sec: rdenv("API_URL")?.parse::<u32>()?
            }
        )
    }
}
