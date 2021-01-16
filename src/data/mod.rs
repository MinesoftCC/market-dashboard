pub mod errors;
pub mod image;
pub mod item;
pub mod states;
pub mod user;

use std::{collections::HashMap, env};

pub type MarketItems = HashMap<String, item::MarketItem>;

lazy_static! {
    pub static ref BANK_API: String = env::var("BANK_API_ADDR").unwrap();
    pub static ref MARKET_API: String = env::var("MARKET_API_ADDR").unwrap();
}
