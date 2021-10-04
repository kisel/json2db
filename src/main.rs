use std::{env, error::Error};
use tokio_postgres::{NoTls};
use tokio::time;

static DB_INIT: &str = r#"
CREATE TABLE IF NOT EXISTS jsonstats (
    id serial not null,
    key text not null,
    data jsonb,
    timestamp timestamp default current_timestamp not null
  )
"#;

async fn insert_record(payload: &str) -> Result<(), Box<dyn Error> > {
    let postgres_url = env::var("DATABASE_URL")?;
    let key = env::var("KEY")?;
    let (client, connection) =
        tokio_postgres::connect(&postgres_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.execute(DB_INIT, &[]).await?;

    let res = client
        .execute("INSERT into jsonstats(key, data) VALUES ($1, $2::TEXT::jsonb)", &[&key, &payload])
        .await;

    match res {
        Ok(_) => println!("Successfully added"),
        Err(e) => println!("Failed: {}", e),
    }

    Ok(())
}

async fn stats_to_db() -> Result<(), Box<dyn Error>> {
    let api_url = env::var("API_URL")?;
    let resp = reqwest::get(api_url)
        .await?
        .text()
        .await?;
    println!("{}", resp);
    insert_record(&resp).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        stats_to_db().await?;
        time::sleep(time::Duration::from_secs(60)).await;
    }
}

