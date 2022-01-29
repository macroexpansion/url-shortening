/* use url_shortening::key_gen;
use url_shortening::schema::{User}; */
use url_shortening::database::Database;

#[tokio::main]
async fn main() {
    let database = Database::new().await.unwrap();

    // database.init().await.unwrap();

    // database.insert("db.users", "(id, name)", "(0, 'test')").await.unwrap();
    database.select("*", "db.users", "").await.unwrap();
}
