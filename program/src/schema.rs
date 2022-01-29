use scylla::macros::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct URL {
    pub hash: String,
    pub original_url: String,
    pub user_id: i32,
}
