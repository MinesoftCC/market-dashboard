use crate::data::{
    states::*,
    statics::{BANK_ADDR, MARKET_ADDR},
};

pub struct ConfigPage;

impl ConfigPage {
    pub fn draw(
        ctx: &egui::CtxRef,
        market_ip: &mut String,
        market_port: &mut String,
        _next_state: &mut State,
    ) {
        #[derive(serde::Deserialize)]
        struct BankResponse {
            ip: String,
            port: i32,
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Create a new config");

            ui.horizontal(|ui| {
                ui.label("Market address");
                ui.text_edit_singleline(market_ip);
                ui.label("(Format: xxx.xxx.xxx.xxx or www.xxxxx.com)");
            });

            ui.horizontal(|ui| {
                ui.label("Market port");
                ui.text_edit_singleline(market_port);
                ui.label("Default: 8000");
            });

            if ui
                .add(
                    egui::Button::new("Create configuration")
                        .enabled(!market_ip.is_empty()),
                )
                .clicked
            {
                let BankResponse { ip, port } = serde_json::from_str(
                    if let Ok(v) = reqwest::blocking::get(
                        format!("https://{}:{}/get_bank_ip", market_ip, market_port)
                            .as_str(),
                    ) {
                        v.text().unwrap()
                    } else {
                        return;
                    }
                    .as_str(),
                )
                .unwrap_or(BankResponse {
                    ip: "0.0.0.0".into(),
                    port: 0,
                });

                if port != 0 {
                    *MARKET_ADDR.lock().unwrap() =
                        format!("http:{}:{}", market_ip, market_port);
                    *BANK_ADDR.lock().unwrap() = format!("http:{}:{}", ip, port);
                }
            }
        });
    }
}
