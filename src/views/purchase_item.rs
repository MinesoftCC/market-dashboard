pub struct PurchasePage;

use crate::data::{
    errors::PurchaseError,
    item::{ItemRatio, MarketItem},
    states::*,
    *,
};

impl PurchasePage {
    pub fn draw(
        ctx: &egui::CtxRef,
        (username, password, confirm_pass, selected_account): (
            &str,
            &mut String,
            &mut String,
            &mut String,
        ),
        password_colour: &mut egui::Color32,
        show_password: &mut bool,
        (next_state, account_status): (&mut State, &mut AccountState),
        (item, purchase_amt): (&mut MarketItem, &mut u32),
        show_purchase_error: &mut PurchaseError,
    ) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        let MarketItem {
            display_name,
            item_id,
            uid,
            price,
            quantity: stock,
            item_ratio,
            poster_id,
            ..
        } = item;

        egui::CentralPanel::default().show(ctx, |ui| match account_status {
            AccountState::LoggedOut => {
                *next_state = State::Market(account_status.clone());
            },
            AccountState::LoggedIn => {
                ui.heading(format!(
                    "Purchasing '{}' posted by {}",
                    display_name,
                    super::get_name_from_id(*poster_id)
                ));

                let sixteen_stack_ids = [
                    "minecraft:snowball",
                    "minecraft:bucket",
                    "minecraft:egg",
                    "minecraft:oak_sign",
                    "minecraft:spruce_sign",
                    "minecraft:birch_sign",
                    "minecraft:jungle_sign",
                    "minecraft:acacia_sign",
                    "minecraft:dark_oak_sign",
                    "minecraft:crimson_sign",
                    "minecraft:warped_sign",
                    "minecraft:ender_pearl",
                    "minecraft:honey_bottle",
                ];

                let multiplier = match item_ratio {
                    ItemRatio::Pair => 2,
                    ItemRatio::HalfStack => {
                        if sixteen_stack_ids.contains(&item_id.as_str()) {
                            8
                        } else {
                            32
                        }
                    },
                    ItemRatio::Stack =>
                        if sixteen_stack_ids.contains(&item_id.as_str()) {
                            16
                        } else {
                            64
                        },
                    ItemRatio::Custom(amt) => *amt,
                    _ => 1,
                };

                let mut current_account = Default::default();
                let max_purchasable = *stock / multiplier;
                let cost = *price * *purchase_amt;

                ui.label("Account:");
                ui.horizontal_wrapped(|ui| {
                    let lock = USER_DATA.lock().unwrap();
                    let mut accounts: Vec<&String> = lock.accounts.keys().collect();

                    accounts.sort();

                    ui.selectable_value(selected_account, "".into(), "None");

                    accounts.iter().for_each(|account_name| {
                        let account_name = (*account_name).clone();
                        let account_data = lock.accounts.get(&account_name).unwrap();
                        ui.selectable_value(
                            selected_account,
                            account_name.clone(),
                            format!(
                                "{} ({} diamonds)",
                                account_name, account_data.balance
                            ),
                        );
                    });
                });

                if !selected_account.is_empty() {
                    let lock = USER_DATA.lock().unwrap();
                    current_account =
                        if let Some(acct) = lock.accounts.get(selected_account) {
                            acct.clone()
                        } else {
                            return;
                        };
                } else {
                    *purchase_amt = 0;
                }

                ui.horizontal(|ui| {
                    if ui.button("+").clicked && *purchase_amt < max_purchasable {
                        *purchase_amt = purchase_amt.wrapping_add(1);
                    }

                    if ui.button(" - ").clicked && *purchase_amt > 0 {
                        *purchase_amt = purchase_amt.wrapping_sub(1);
                    }

                    let purchase_amt_clone = *purchase_amt;

                    ui.add(
                        egui::Slider::u32(purchase_amt, 0..=max_purchasable)
                            .text_color(
                                if cost > current_account.balance as u32 {
                                    egui::Color32::RED
                                } else if cost == 0 {
                                    egui::Color32::from_rgb(255, 165, 0)
                                } else {
                                    egui::Color32::GREEN
                                },
                            )
                            .text(
                                if *item_ratio != ItemRatio::Individual {
                                    format!(
                                        "{}(s) out of a possible {}. ({} {}(s) = {} \
                                         {}(s))",
                                        item_ratio,
                                        max_purchasable,
                                        purchase_amt_clone,
                                        item_ratio,
                                        purchase_amt_clone * multiplier,
                                        display_name.to_lowercase(),
                                    )
                                } else {
                                    format!(
                                        "{}(s) out of a possible {}",
                                        item_ratio, max_purchasable
                                    )
                                },
                            ),
                    );
                });

                ui.colored_label(
                    if cost > current_account.balance as u32 {
                        egui::Color32::RED
                    } else if cost == 0 {
                        egui::Color32::from_rgb(255, 165, 0)
                    } else {
                        egui::Color32::GREEN
                    },
                    format!("Price: {} diamond(s)", cost),
                );

                if selected_account.is_empty() {
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "A bank account must be selected",
                    );
                }

                if *purchase_amt == 0 {
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "Cannot purchase 0 of any item",
                    );
                }

                ui.heading("");

                egui::CollapsingHeader::new("Confirm details")
                    .default_open(false)
                    .show(ui, |ui| {
                        let mut enable_purchase = false;

                        ui.horizontal(|ui| {
                            ui.label("Enter password");
                            ui.add(
                                egui::TextEdit::singleline(password)
                                    .text_color(*password_colour),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Confirm password");
                            ui.add(
                                egui::TextEdit::singleline(confirm_pass)
                                    .text_color(*password_colour),
                            );
                            ui.checkbox(show_password, "Show password");
                        });

                        enable_purchase |= !password.is_empty()
                            && !confirm_pass.is_empty()
                            && password == confirm_pass
                            && *purchase_amt != 0
                            && *price <= current_account.balance as u32;

                        if ui
                            .add(
                                egui::Button::new("Complete purchase")
                                    .enabled(enable_purchase),
                            )
                            .clicked
                        {
                            #[derive(serde::Deserialize)]
                            enum PurchaseResonse {
                                Success,
                                Fail(String),
                            };

                            let client = reqwest::blocking::Client::new();
                            let response: PurchaseResonse = serde_json::from_str(
                                client
                                    .post(
                                        format!(
                                            "{}/buy/{}/{}/{}/{}",
                                            MARKET_API.to_string(),
                                            uid,
                                            USER_DATA.get_user_id(),
                                            username,
                                            cost
                                        )
                                        .as_str(),
                                    )
                                    .header(
                                        reqwest::header::CONTENT_TYPE,
                                        password.clone(),
                                    )
                                    .send()
                                    .unwrap()
                                    .text()
                                    .unwrap()
                                    .as_str(),
                            )
                            .unwrap();

                            if let PurchaseResonse::Fail(message) = response {
                                *show_purchase_error = PurchaseError::Show(message);
                            } else {
                                MARKET_DATA.update();
                            }
                        };

                        if password.is_empty() {
                            ui.colored_label(
                                egui::Color32::YELLOW,
                                "Password cannot be empty",
                            );
                        }

                        if confirm_pass.is_empty() {
                            ui.colored_label(
                                egui::Color32::YELLOW,
                                "Confirm password cannot be empty",
                            );
                        }
                    });
            },
        });
    }
}
