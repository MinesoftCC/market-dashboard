#![forbid(unsafe_code)]
#![warn(clippy::all)]

pub mod app;
pub mod views;
pub use app::MarketDashboard;

fn main() {
    let app = MarketDashboard::default();
    eframe::run_native(Box::new(app));
}
