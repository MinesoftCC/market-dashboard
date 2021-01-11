pub mod errors;
pub mod item;
pub mod states;
pub mod user;

use std::collections::HashMap;

pub type MarketItems = HashMap<String, item::MarketItem>;
