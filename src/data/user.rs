#[derive(Default, Clone)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub balance: i32,
    pub perms: i32,
}
