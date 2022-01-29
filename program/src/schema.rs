
#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct URL {
    pub hash: String,
    pub original_url: String,
    pub user_id: u32,
}
