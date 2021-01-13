use super::image::Image;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MarketItem {
    pub item_id: String,
    pub item_image_url: String,
    pub display_name: String,
    pub quantity: u32,
    pub price: u32,
    pub poster_id: u16,
    pub time_posted: String,
    #[serde(skip)]
    pub image: Image,
}
