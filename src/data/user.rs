use std::collections::HashMap;

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct AccountInner {
    pub balance: i32,
    clearance: i32,
}

pub type UserAccounts = HashMap<String, AccountInner>;

#[derive(Default, Clone)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub balance: i32,
    pub perms: i32,
    pub accounts: UserAccounts,
}
