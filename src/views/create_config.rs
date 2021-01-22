use crate::data::states::*;

pub struct ConfigPage;

impl ConfigPage {
    pub fn draw(
        ctx: &egui::CtxRef,
        market_ip: &mut String,
        market_port: &mut String,
        _next_state: &mut State,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Create a new config");

            ui.horizontal(|ui| {
                ui.label("Market IP");
                ui.text_edit_singleline(market_ip);
            });

            ui.horizontal(|ui| {
                ui.label("Market port");
                ui.text_edit_singleline(market_port);
                ui.label("Default: 8000");
            });
        });
    }
}
