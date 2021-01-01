#![forbid(unsafe_code)]
#![warn(clippy::all)]

pub mod app;
pub use app::EguiApp;

fn main() {
    let app = EguiApp::default();
    egui_glium::run(Box::new(app));
}
