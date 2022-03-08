use std::{env, error::Error};
use anyhow::{Context, Result};
use structopt::StructOpt;
use serde::{Deserialize};



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
    #[structopt(long, env = "API_URL", default_value="")]
    pub api_url: String,

    /// Default top-level scrape interval in seconds
    #[structopt(short, long, env = "INTERVAL", default_value = "60")]
    pub interval_sec: u32,

    /// scratch config yamj
    #[structopt(short, long, env = "CONFIG", default_value="")]
    pub config: String,
}

pub struct DbConfig {
    /// Target database URL
    pub database_url: String,

    /// Database record group key
    pub key: String,
}


#[derive(Deserialize, Default, Clone)]
pub struct ConfigYaml {
    pub instances: Vec<ScratchItem>,

    // in case 0 - interval should be taken from upper level
    #[serde(default = "unset_interval")]
    pub interval: u32,
}

fn unset_interval() -> u32 { 0 }

#[derive(Deserialize, Default, Clone)]
pub struct ScratchItem {
    pub key: String,
    pub url: String,

    #[serde(default = "unset_interval")]
    pub interval: u32,
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
                interval_sec: rdenv("API_URL")?.parse::<u32>()?,
                config: "".to_string(),
            }
        )
    }

    pub async fn read_config_file(&self) -> Result<ConfigYaml> {
        if self.config == "" {
            return Ok(ConfigYaml::default());
        }
        let f = std::fs::File::open(&self.config)?;
        let cf: ConfigYaml = serde_yaml::from_reader(f)?;
        return Ok(cf);
    }

    /// returns optional single scratch item from CLI
    pub fn get_scratch_item(&self) -> Option<ScratchItem> {
        if self.api_url != "" && self.key != "" {
            Some(ScratchItem{key: self.key.clone(), url: self.api_url.clone(), interval: 0})
        } else {
            None
        }
    }

    pub fn get_db_config(&self) -> DbConfig {
        DbConfig{
            database_url: self.database_url.clone(),
            key: self.key.clone(),
        }
    }
}
