use url_shortening::key_gen;
use url_shortening::schema::{User};

fn main() {
    println!("Hello, world!");

    let user = User {
        id: 1,
        name: "test".to_string()
    };

    println!("{user:?}");

    key_gen::key_gen();
}
