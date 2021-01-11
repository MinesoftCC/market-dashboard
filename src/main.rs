#![forbid(unsafe_code)]
#![warn(clippy::all)]

#[macro_use]
extern crate lazy_static;

pub mod app;
pub mod data;
pub mod views;

use crate::app::MarketDashboard;

fn main() {
    let app = MarketDashboard::default();
    eframe::run_native(Box::new(app));
}
