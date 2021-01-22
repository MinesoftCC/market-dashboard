use super::image::Image;
use std::fmt::{self, Display};

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Debug)]
pub enum ItemRatio {
    Individual,
    Pair,
    HalfStack,
    Stack,
    Custom(u32),
}

impl Default for ItemRatio {
    fn default() -> Self { Self::Individual }
}

impl Display for ItemRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match self {
            Self::Individual => "item".into(),
            Self::Pair => "pair".into(),
            Self::HalfStack => "half-stack".into(),
            Self::Stack => "stack".into(),
            Self::Custom(amt) => format!("{} items", amt),
        };

        write!(f, "{}", display)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MarketItem {
    #[serde(skip)]
    pub uid: String,
    #[serde(skip)]
    pub deleted: bool,
    pub item_id: String,
    pub item_image_url: String,
    pub display_name: String,
    pub quantity: u32,
    pub price: u32,
    pub poster_id: u16,
    pub time_posted: String,
    pub item_ratio: ItemRatio,
    #[serde(skip)]
    pub image: Image,
}

impl MarketItem {
    pub fn to_sendable(&self) -> HttpItem {
        let Self {
            item_id,
            item_image_url,
            display_name,
            quantity,
            price,
            poster_id,
            time_posted,
            item_ratio: ratio,
            ..
        } = self.clone();

        HttpItem {
            item_id,
            item_image_url,
            display_name,
            quantity,
            price,
            poster_id,
            time_posted,
            item_ratio: ratio,
        }
    }
}

impl PartialEq for MarketItem {
    fn eq(&self, other: &Self) -> bool {
        self.display_name == other.display_name
            && self.item_id == other.item_id
            && self.item_image_url == other.item_image_url
            && self.item_ratio == other.item_ratio
            && self.poster_id == other.poster_id
            && self.price == other.price
            && self.quantity == other.quantity
            && self.time_posted == other.time_posted
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HttpItem {
    pub item_id: String,
    pub item_image_url: String,
    pub display_name: String,
    pub quantity: u32,
    pub price: u32,
    pub poster_id: u16,
    pub time_posted: String,
    pub item_ratio: ItemRatio,
}
