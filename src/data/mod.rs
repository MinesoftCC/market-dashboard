pub mod errors;
pub mod image;
pub mod item;
pub mod states;
pub mod statics;
pub mod user;

use errors::BankConnectionError;
use std::{collections::HashMap, env};

pub use statics::*;

pub type MarketItems = HashMap<String, item::MarketItem>;

pub fn get_user_id(username: &str) -> i32 {
    let client = reqwest::blocking::Client::new();
    let mut user_id = 0;

    let response = match client
        .get(format!("{}/listusers", BANK_API.to_string()).as_str())
        .send()
    {
        Ok(v) => v,
        Err(_) => {
            *BANK_CONNECTION_ERROR.lock().unwrap() = BankConnectionError::Show(
                "Could not connect to bank server to get user ID".into(),
            );
            return user_id;
        },
    };

    let users: Vec<String> =
        if let Ok(v) = serde_json::from_str(response.text().unwrap().as_str()) {
            v
        } else {
            Vec::new()
        };

    users.iter().enumerate().into_iter().for_each(|(id, user)| {
        if user == username {
            user_id = (id) as i32;
        }
    });

    USER_DATA.lock().unwrap().id = user_id;

    user_id
}

#[cfg(not(debug_assertions))]
pub static BANK_API: &str = env!("BANK_API_ADDR", "Please define BANK_API_ADDR");
#[cfg(not(debug_assertions))]
pub static MARKET_API: &str = env!("MARKET_API_ADDR", "Please define MARKET_API_ADDR");
#[cfg(not(debug_assertions))]
pub static ADMIN_PASS: &str = env!("ADMIN_PASS", "Please define ADMIN_PASS");

#[cfg(debug_assertions)]
lazy_static! {
    pub static ref BANK_API: String = env::var("BANK_API_ADDR").unwrap();
    pub static ref MARKET_API: String = env::var("MARKET_API_ADDR").unwrap();
    pub static ref ADMIN_PASS: String = env::var("ADMIN_PASS").unwrap();
}
