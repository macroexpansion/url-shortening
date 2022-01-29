use anyhow::{Result, anyhow};
use scylla::{SessionBuilder, Session};
use crate::schema::{User, URL};
use scylla::IntoTypedRows;

pub struct Database {
    session: Session
}

impl Database {
    pub async fn new() -> Result<Self> {
        let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

        println!("Connecting to {uri}");

        let session: Session = SessionBuilder::new().known_node(uri).build().await?;

        Ok(Database {
            session: session
        })
    }

    pub async fn init(&self) -> Result<()> {
        self.session.query(
            "CREATE KEYSPACE IF NOT EXISTS db WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : '1' }",
            &[]
        ).await?;

        self.session.query(
            "CREATE TABLE IF NOT EXISTS db.users ( id int, name text, PRIMARY KEY (id) )",
            &[]
        ).await?;

        self.session.query(
            "CREATE TABLE IF NOT EXISTS db.urls ( hash text, original_url text, user_id int, PRIMARY KEY (hash) )",
            &[]
        ).await?;

        self.session.query(
            "CREATE INDEX ON db.urls ( original_url )",
            &[]
        ).await?;

        Ok(())
    }
    
    pub async fn select(&self, columns: &str, table: &str, condition: &str) -> Result<()> {
        // let query = if columns == "*" {format!("SELECT {columns} FROM {table}")} else {format!("SELECT {columns} FROM {table} WHERE {condition}")};
        let query = format!("SELECT {columns} FROM {table} WHERE {condition}");

        if let Some(rows) = self.session.query(query, &[]).await?.rows {
            if table == "db.users" {
                for row in rows.into_typed::<User>() {
                    let row = row?;
                    println!("database {row:?}");
                    return Ok(());
                }
            } else {
                for row in rows.into_typed::<URL>() {
                    let row = row?;
                    println!("database {row:?}");
                    return Ok(());
                }
            }
        }

        Err(anyhow!("not found"))
    }

    pub async fn insert(&self, table: &str, columns: &str, values: &str) -> Result<()> {
        self.session.query(
            format!("
                INSERT INTO {table} {columns} VALUES {values}
            "),
            &[]
        ).await?;

        Ok(())
    }
}
