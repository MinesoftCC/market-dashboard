pub mod errors;
pub mod image;
pub mod item;
pub mod states;
pub mod user;

use std::{collections::HashMap, env};

pub type MarketItems = HashMap<String, item::MarketItem>;

#[cfg(not(debug_assertions))]
pub static BANK_API: &'static str = env!("BANK_API_ADDR", "Please define BANK_API_ADDR");
#[cfg(not(debug_assertions))]
pub static MARKET_API: &'static str =
    env!("MARKET_API_ADDR", "Please define MARKET_API_ADDR");
#[cfg(not(debug_assertions))]
pub static ADMIN_PASS: &'static str = env!("ADMIN_PASS", "Please define ADMIN_PASS");

#[cfg(debug_assertions)]
lazy_static! {
    pub static ref BANK_API: String = env::var("BANK_API_ADDR").unwrap();
    pub static ref MARKET_API: String = env::var("MARKET_API_ADDR").unwrap();
    pub static ref ADMIN_PASS: String = env::var("ADMIN_PASS").unwrap();
}
