use url_shortening::key_gen::key_gen;
use url_shortening::database::Database;
use anyhow::Result;

extern crate redis;
use crate::redis::{Commands, Connection};

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let database = Database::new().await.unwrap();
    // database.init().await.unwrap();
    add_url(&database, "https://google.com").await.unwrap();

    processor(&mut con, &database, "aaaaab").await.unwrap();
    processor(&mut con, &database, "aaaaaa").await.unwrap();
    processor(&mut con, &database, "aaaaaa").await.unwrap();
}

async fn processor(cache: &mut Connection, database: &Database, hash: &str) -> Result<()> {
    match find_url(cache, &database, hash).await {
        Ok(()) => {},
        Err(error) => println!("{error:?}")
    };

    Ok(())
}

async fn find_url(cache: &mut Connection, database: &Database, hash: &str) -> Result<()> {
    // find in cache
    match cache.get::<String, String>(hash.to_string()) {
        Ok(value) => {
            println!("cache {value}");
            return Ok(());
        },
        Err(_) => {}
    };

    // find in database
    database.select("*", "db.urls", &format!("hash = '{hash}'")).await?;

    // update cache
    cache.set("aaaaaa", "https://google.com")?;
    println!("updated cache");

    Ok(())
}

async fn add_url(database: &Database, url: &str) -> Result<()> {
    let hash = key_gen();
    let user_id = 0;
    let url_values = format!("('{hash}', '{url}', {user_id})");
    database.insert("db.urls", "(hash, original_url, user_id)", &url_values).await?;

    Ok(())
}
