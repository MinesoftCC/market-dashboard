use crate::data::{errors::*, item::*, states::*, *};
use chrono::prelude::*;

pub struct IndexPage;

impl IndexPage {
    fn main_content(
        ui: &mut egui::Ui,
        frame: &mut epi::Frame<'_>,
        search_term: &mut String,
        (password, confirm_pass, current_account): (
            &mut String,
            &mut String,
            &mut String,
        ),
        refresh: &mut bool,
        (next_state, account_status, delete_prompt_state): (
            &mut State,
            &mut AccountState,
            &mut DeletePromptState,
        ),
        app_item: &mut MarketItem,
    ) {
        ui.vertical_centered(|ui| {
            let mut market_data = MARKET_DATA.lock().unwrap();

            let mut items = market_data
                .iter_mut()
                .map(|(key, item)| {
                    item.uid = key.clone();
                    item
                })
                .collect::<Vec<&mut MarketItem>>();

            items.sort_by(|a, b| {
                let a_dt = DateTime::parse_from_rfc2822(a.time_posted.as_str())
                    .expect("Could not parse datetime from post date");
                let b_dt = DateTime::parse_from_rfc2822(b.time_posted.as_str())
                    .expect("Could not parse datetime from post date");

                a_dt.cmp(&b_dt)
            });

            if !search_term.is_empty() {
                items = items
                    .into_iter()
                    .filter(|item| {
                        item.display_name
                            .to_lowercase()
                            .contains(&search_term.to_lowercase())
                    })
                    .collect();
            }

            items.iter_mut().for_each(|item| {
                let mut clicked = false;
                let (_, mut response) = ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        if let Some(texture_id) = item.image.as_texture(frame) {
                            let size = egui::Vec2::new(
                                item.image.size.0.min(100) as f32,
                                item.image.size.1.min(100) as f32,
                            );

                            ui.image(texture_id, size);
                        }

                        ui.vertical(|ui| {
                            clicked |= ui.heading(&item.display_name).clicked;
                            clicked |= ui
                                .colored_label(
                                    egui::Color32::LIGHT_GRAY,
                                    format!("In-game id: {}", item.item_id),
                                )
                                .clicked;
                            clicked |= ui
                                .colored_label(
                                    egui::Color32::LIGHT_GRAY,
                                    format!(
                                        "Price: {} diamond(s) each {}",
                                        item.price, item.item_ratio,
                                    ),
                                )
                                .clicked;
                            clicked |= ui
                                .colored_label(
                                    egui::Color32::LIGHT_GRAY,
                                    format!("Quantity: {}", item.quantity),
                                )
                                .clicked;
                            clicked |= ui
                                .colored_label(
                                    egui::Color32::LIGHT_GRAY,
                                    format!(
                                        "Time posted: {}",
                                        DateTime::parse_from_rfc2822(
                                            item.time_posted.as_str()
                                        )
                                        .expect("Could not parse datetime from post date")
                                        .naive_local()
                                    ),
                                )
                                .clicked;

                            clicked |= ui
                                .colored_label(
                                    egui::Color32::LIGHT_GRAY,
                                    format!(
                                        "Posted by: {}",
                                        super::get_name_from_id(item.poster_id)
                                    ),
                                )
                                .clicked;

                            ui.horizontal(|ui| {
                                let same_user =
                                    item.poster_id == USER_DATA.get_user_id() as u16;

                                if same_user && ui.button("Delete").clicked {
                                    *delete_prompt_state =
                                        DeletePromptState::Show(item.uid.clone());
                                }

                                if same_user && ui.button("Edit").clicked {
                                    *app_item = item.clone();
                                    *next_state = State::EditItem(account_status.clone());
                                }

                                if !same_user
                                    && *account_status == AccountState::LoggedIn
                                    && ui.button("Purchase").clicked
                                {
                                    *app_item = item.clone();
                                    *password = "".into();
                                    *confirm_pass = "".into();
                                    *current_account = "".into();
                                    *next_state =
                                        State::PurchaseItem(account_status.clone());
                                }

                                #[cfg(debug_assertions)]
                                if ui.button("Copy UID").clicked {
                                    use clipboard::{
                                        ClipboardContext, ClipboardProvider,
                                    };

                                    let mut clip_ctx: ClipboardContext =
                                        ClipboardProvider::new().unwrap();
                                    clip_ctx.set_contents(item.uid.clone()).unwrap();
                                    println!("Copied '{}' to clipboard", item.uid);
                                }
                            });

                            if let DeletePromptState::Show(uid) = delete_prompt_state {
                                if item.uid == *uid {
                                    ui.label("Are you sure?");
                                    ui.horizontal(|ui| {
                                        if ui.button("Yes").clicked {
                                            let client = reqwest::blocking::Client::new();
                                            let _response = client
                                                .post(
                                                    format!(
                                                        "{}/remove_item",
                                                        MARKET_API.to_string()
                                                    )
                                                    .as_str(),
                                                )
                                                .header(
                                                    reqwest::header::CONTENT_TYPE,
                                                    "application/json",
                                                )
                                                .body(
                                                    serde_json::to_string(&item.uid)
                                                        .unwrap(),
                                                )
                                                .send()
                                                .unwrap();

                                            *refresh = true;
                                            item.deleted = true;
                                        }
                                        if ui.button("No").clicked {
                                            *delete_prompt_state =
                                                DeletePromptState::Hide;
                                        }
                                    });
                                }
                            }
                        });
                    });
                    ui.separator();
                });

                response.id = egui::Id::new(item.uid.clone());
                response = response.interact(egui::Sense::click());

                if response.clicked || clicked {
                    *next_state = State::Item(account_status.clone(), item.clone());
                }

                response.on_hover_text(format!(
                    "Click to go to the page for {}",
                    item.display_name
                ));
            });
        });
    }

    pub fn draw(
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        (username, search_term, password, confirm_pass, current_account): (
            &str,
            &mut String,
            &mut String,
            &mut String,
            &mut String,
        ),
        refresh: &mut bool,
        (account_status, next_state, delete_prompt_state): (
            &mut AccountState,
            &mut State,
            &mut DeletePromptState,
        ),
        app_item: &mut MarketItem,
    ) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        let market_connection_error = MARKET_CONNECTION_ERROR.lock().unwrap().clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if *refresh || ui.button("Refresh market").clicked {
                    *refresh = false;
                    MARKET_DATA.update();
                    frame.repaint_signal().request_repaint();
                }

                ui.label("Search: ");
                ui.text_edit_singleline(search_term);

                if ui.small_button(" X ").clicked {
                    *search_term = "".into();
                }
            });
            ui.separator();

            let is_empty = MARKET_DATA.lock().unwrap().is_empty();

            match (market_connection_error, is_empty) {
                (MarketConnectionError::Show(message), _) => {
                    ui.vertical_centered_justified(|ui| {
                        ui.colored_label(egui::Color32::RED, message.clone());
                    });
                },
                (MarketConnectionError::Hide, true) => {
                    ui.vertical_centered(|ui| {
                        ui.heading("No items");
                    });
                },
                (MarketConnectionError::Hide, false) => {
                    egui::ScrollArea::auto_sized().show(ui, |ui| {
                        Self::main_content(
                            ui,
                            frame,
                            search_term,
                            (password, confirm_pass, current_account),
                            refresh,
                            (next_state, account_status, delete_prompt_state),
                            app_item,
                        )
                    });
                },
            }
        });
    }
}
