use std::{error::Error};
use ::futures::future;
use structopt::StructOpt;
use tokio_postgres::{NoTls};
use tokio::{time};
use anyhow::{Result};

mod config;
use config::{Config, DbConfig, ScratchItem};

static DB_INIT: &str = r#"
CREATE TABLE IF NOT EXISTS jsonstats (
    id serial not null,
    key text not null,
    data jsonb,
    timestamp timestamp default current_timestamp not null
  )
"#;

async fn insert_record(cfg: &DbConfig, payload: &str) -> Result<(), Box<dyn Error> > {
    let (client, connection) =
        tokio_postgres::connect(&cfg.database_url, NoTls).await?;

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

async fn stats_to_db(cfg: &DbConfig, scratch: &ScratchItem) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get(&scratch.url)
        .await?
        .text()
        .await?;
    println!("{}", resp);
    insert_record(&cfg, &resp).await?;
    Ok(())
}

async fn start_worker(cfg: &Config, scratch: ScratchItem) {
    let dbcfg = cfg.get_db_config();
    let interval: u32 = if scratch.interval != 0 {
        scratch.interval
    } else {
        cfg.interval_sec
    };
    loop {
        match stats_to_db(&dbcfg, &scratch).await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Request has failed: {}", e)
        }
        println!("Next tick in {} sec", interval);
        time::sleep(time::Duration::from_secs(interval.into())).await;
    }
}

async fn run_workers(cfg: &Config) -> Result<()> {
    let filecfg = cfg.read_config_file().await?;

    let cli_scratch_item = &cfg.get_scratch_item();

    let mut workers = Vec::new();
    for sii in filecfg.instances.iter().chain(cli_scratch_item) {
        workers.push(start_worker(cfg, (*sii).clone()));
    }
    future::join_all(workers).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    run_workers(&Config::from_args())
        .await
        .unwrap_or_else(|e| println!("Whoops: {}", e));
}

