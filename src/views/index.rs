use crate::{
    app::{MARKET_CONNECTION_ERROR, MARKET_DATA},
    data::{errors::*, item::*, states::*},
};
use chrono::prelude::*;

pub struct IndexPage;

impl IndexPage {
    pub fn draw(ctx: &egui::CtxRef, username: &str, account_status: &mut AccountState, next_state: &mut State) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        let market_connection_error = MARKET_CONNECTION_ERROR.lock().unwrap().clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Refresh market").clicked {
                    MARKET_DATA.update();
                }

                // ui.text_edit_singleline();
            });
            ui.separator();

            match market_connection_error {
                MarketConnectionError::Show(message) => {
                    ui.vertical_centered_justified(|ui| {
                        ui.colored_label(egui::color::Color32::RED, message.clone());
                    });
                },
                MarketConnectionError::Hide => {
                    ui.horizontal_wrapped(|ui| {
                        let market_data = MARKET_DATA.lock().unwrap();

                        let mut items = market_data
                            .values()
                            .map(|item| item.clone())
                            .collect::<Vec<MarketItem>>();

                        items.sort_by(|a, b| {
                            let a_dt = DateTime::parse_from_rfc2822(a.time_posted.as_str())
                                .expect("Could not parse datetime from post date");
                            let b_dt = DateTime::parse_from_rfc2822(b.time_posted.as_str())
                                .expect("Could not parse datetime from post date");

                            a_dt.cmp(&b_dt)
                        });

                        items.iter().for_each(|item| {
                            let mut clicked = false;
                            let (_, mut response) = ui.vertical(|ui| {
                                let item = item.clone();

                                clicked |= ui.heading(&item.display_name).clicked;
                                clicked |= ui
                                    .colored_label(egui::Color32::LIGHT_GRAY, format!("In-game id: {}", item.item_id))
                                    .clicked;
                                clicked |= ui
                                    .colored_label(egui::Color32::LIGHT_GRAY, format!("Price: {} per item", item.price))
                                    .clicked;
                                clicked |= ui
                                    .colored_label(egui::Color32::LIGHT_GRAY, format!("Quantity: {}", item.quantity))
                                    .clicked;
                                clicked |= ui
                                    .colored_label(
                                        egui::Color32::LIGHT_GRAY,
                                        format!(
                                            "Time posted: {}",
                                            DateTime::parse_from_rfc2822(item.time_posted.as_str())
                                                .expect("Could not parse datetime from post date")
                                                .naive_local()
                                        ),
                                    )
                                    .clicked;
                            });

                            response.id =
                                egui::Id::new(format!("{}{}{}", item.poster_id, item.item_id, item.time_posted));
                            response = response.interact(egui::Sense::click());

                            if response.clicked || clicked {
                                *next_state = State::ItemPage(account_status.clone(), item.clone());
                            }

                            response.on_hover_ui(|ui| {
                                ui.label(format!("Click to go to the page for {}", item.display_name));
                            });
                        });
                    });
                },
            }
        });
    }
}
