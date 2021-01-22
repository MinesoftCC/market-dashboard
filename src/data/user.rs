use std::collections::HashMap;

#[derive(serde::Deserialize, Clone, Debug, Default)]
pub struct AccountInner {
    pub balance: i32,
    clearance: i32,
}

pub type UserAccounts = HashMap<String, AccountInner>;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub balance: i32,
    pub perms: i32,
    pub accounts: UserAccounts,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: u16::MAX as i32,
            balance: Default::default(),
            perms: Default::default(),
            username: Default::default(),
            accounts: Default::default(),
        }
    }
}
