use std::{error::Error};
use tokio_postgres::{NoTls};
use tokio::time;
use anyhow::{Result};

mod config;
use config::Config;

static DB_INIT: &str = r#"
CREATE TABLE IF NOT EXISTS jsonstats (
    id serial not null,
    key text not null,
    data jsonb,
    timestamp timestamp default current_timestamp not null
  )
"#;

async fn insert_record(cfg: &Config, payload: &str) -> Result<(), Box<dyn Error> > {
    let (client, connection) =
        tokio_postgres::connect(&cfg.postgres_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(DB_INIT, &[]).await?;

    let res = client
        .execute("INSERT into jsonstats(key, data) VALUES ($1, $2::TEXT::jsonb)", &[&cfg.key, &payload])
        .await;

    match res {
        Ok(_) => println!("Successfully added"),
        Err(e) => println!("Failed: {}", e),
    }

    Ok(())
}

async fn stats_to_db(cfg: &Config) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(&cfg.api_url)
        .await?
        .text()
        .await?;
    println!("{}", resp);
    insert_record(&cfg, &resp).await?;
    Ok(())
}

async fn start_loop() -> Result<(), Box<dyn Error>> {
    const INTERVAL: u32 = 60;
    let cfg = Config::new()?;

    loop {
        match stats_to_db(&cfg).await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Request has failed: {}", e)
        }
        println!("Next tick in {} sec", INTERVAL);
        time::sleep(time::Duration::from_secs(60)).await;
    }
}

#[tokio::main]
async fn main() {
    start_loop()
        .await
        .unwrap_or_else(|e| println!("Whoops: {}", e));
}

