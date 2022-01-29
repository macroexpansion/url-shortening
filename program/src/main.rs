use url_shortening::key_gen::key_gen;
use url_shortening::database::Database;
use anyhow::Result;

extern crate redis;
use crate::redis::{Commands, Connection};

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _ : () = con.set("https://google.com", "aaaaaa").unwrap();

    let database = Database::new().await.unwrap();
    // database.init().await.unwrap();

    processor(&mut con, &database, "https://google.com").await.unwrap();
}

async fn processor(cache: &mut Connection, database: &Database, url: &str) -> Result<()> {
    match find_url(cache, &database, url).await {
        Ok(()) => {},
        Err(error) => {
            if error.to_string() == "not found" {
                shorten_url(database, url).await?;
            } else {
                println!("{error:?}");
            }
        }
    };

    Ok(())
}

async fn find_url(cache: &mut Connection, database: &Database, url: &str) -> Result<()> {
    match cache.get::<String, String>(url.to_string()) {
        Ok(value) => {
            println!("{value}");
            return Ok(());
        },
        Err(_) => {}
    };

    println!("bug");
    
    // database.select("*", "db.urls", &format!("original_url = '{url}'")).await?;

    Ok(())
}

async fn shorten_url(database: &Database, url: &str) -> Result<()> {
    let hash = key_gen();
    let user_id = 0;
    let url_values = format!("('{hash}', '{url}', {user_id})");
    database.insert("db.urls", "(hash, original_url, user_id)", &url_values).await?;

    Ok(())
}
