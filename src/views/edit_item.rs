use crate::data::{
    item::{ItemRatio, MarketItem},
    states::*,
    *,
};

use chrono::prelude::*;

pub struct EditItemPage;

impl EditItemPage {
    pub fn draw(
        ctx: &egui::CtxRef,
        username: &str,
        next_state: &mut State,
        account_status: &mut AccountState,
        item: &mut MarketItem,
        item_ratio: &mut u32,
    ) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        let mut sendable_item = item.clone().to_sendable();
        let MarketItem {
            display_name,
            item_image_url,
            item_id,
            price,
            quantity,
            item_ratio: ratio,
            uid,
            ..
        } = item;

        let name = display_name.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Editing '{}'", name));

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Image URL");
                        ui.text_edit_singleline(item_image_url);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Display name");
                        ui.add(egui::TextEdit::singleline(display_name).enabled(false));
                        ui.label("[locked]");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Item ID");
                        ui.add(egui::TextEdit::singleline(item_id).enabled(false));
                        ui.label("[locked]");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Price");
                        if ui.button(" - ").clicked {
                            *price = price.wrapping_sub(1);
                        }
                        ui.add(egui::Button::new(format!("{}", price)).enabled(false));
                        if ui.button("+").clicked {
                            *price = price.wrapping_add(1);
                        }
                        ui.label("diamonds each");

                        ui.horizontal(|ui| {
                            ui.radio_value(ratio, ItemRatio::Individual, "item");
                            ui.radio_value(ratio, ItemRatio::Pair, "pair");
                            ui.radio_value(ratio, ItemRatio::HalfStack, "half-stack");
                            ui.radio_value(ratio, ItemRatio::Stack, "stack");
                            ui.radio_value(
                                ratio,
                                ItemRatio::Custom(*item_ratio),
                                "custom: ",
                            );

                            if ui.button(" - ").clicked {
                                *item_ratio = item_ratio.wrapping_sub(1);
                                *ratio = ItemRatio::Custom(*item_ratio);
                            }
                            ui.add(
                                egui::Button::new(format!("{}", item_ratio))
                                    .enabled(false),
                            );
                            if ui.button("+").clicked {
                                *item_ratio = item_ratio.wrapping_add(1);
                                *ratio = ItemRatio::Custom(*item_ratio);
                            }
                        });
                    });

                    ui.horizontal(|ui| {
                        ui.label("Quantity");
                        if ui.button(" -64 ").clicked {
                            *quantity = quantity.wrapping_sub(64);
                        }
                        if ui.button(" -32 ").clicked {
                            *quantity = quantity.wrapping_sub(32);
                        }
                        if ui.button(" -16 ").clicked {
                            *quantity = quantity.wrapping_sub(16);
                        }
                        if ui.button(" -10 ").clicked {
                            *quantity = quantity.wrapping_sub(10);
                        }
                        if ui.button(" -1 ").clicked {
                            *quantity = quantity.wrapping_sub(1);
                        }
                        ui.add(egui::Button::new(format!("{}", quantity)).enabled(false));
                        if ui.button("+1").clicked {
                            *quantity = quantity.wrapping_add(1);
                        }
                        if ui.button("+10").clicked {
                            *quantity = quantity.wrapping_add(10);
                        }
                        if ui.button("+16").clicked {
                            *quantity = quantity.wrapping_add(16);
                        }
                        if ui.button("+32").clicked {
                            *quantity = quantity.wrapping_add(32);
                        }
                        if ui.button("+64").clicked {
                            *quantity = quantity.wrapping_add(64);
                        }
                    });
                });

                ui.separator();

                ui.vertical(|ui| {
                    let mut disable_submit = false;

                    if display_name.is_empty() {
                        disable_submit |= true;
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "Display name cannot be empty",
                        );
                    }

                    if item_id.is_empty() {
                        disable_submit |= true;
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "Item ID cannot be empty",
                        );
                    }

                    if *price == 0 {
                        disable_submit |= true;
                        ui.colored_label(egui::Color32::YELLOW, "Price cannot be 0");
                    }

                    if *quantity == 0 {
                        disable_submit |= true;
                        ui.colored_label(egui::Color32::YELLOW, "Quantity cannot be 0");
                    }

                    if let ItemRatio::Custom(amt) = ratio {
                        if *amt == 0 {
                            disable_submit |= true;

                            ui.colored_label(
                                egui::Color32::YELLOW,
                                "Custom item ratio cannot be 0",
                            );
                        }
                    }

                    if ui
                        .add(egui::Button::new("Submit").enabled(!disable_submit))
                        .clicked
                    {
                        sendable_item.time_posted = Utc::now().to_rfc2822();
                        sendable_item.poster_id = USER_DATA.get_user_id() as u16;

                        let client = reqwest::blocking::Client::new();
                        let _response = client
                            .post(
                                format!("{}/edit_item/{}", MARKET_API.to_string(), uid)
                                    .as_str(),
                            )
                            .header(reqwest::header::CONTENT_TYPE, "application/json")
                            .body(serde_json::to_string(&sendable_item).unwrap())
                            .send()
                            .unwrap();

                        MARKET_DATA.update();
                        *next_state = State::Market(account_status.clone());
                    }
                });
            });
        });
    }
}
