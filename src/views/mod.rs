mod index;
mod item_page;
mod login;
mod profile;

pub use index::*;
pub use item_page::*;
pub use login::*;
pub use profile::*;

use crate::data::states::*;

fn draw_sidebar(
    ctx: &egui::CtxRef,
    username: &str,
    next_state: &mut State,
    account_status: &mut AccountState,
) {
    egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
        ui.horizontal_wrapped(|ui| match account_status {
            AccountState::LoggedIn => {
                let mut response =
                    ui.heading(username.to_string()).on_hover_ui(|ui| {
                        ui.label(
                            "Click on your username to go to your profile",
                        );
                    });

                response = response.interact(egui::Sense::click());

                if response.clicked {
                    *next_state = State::Profile(account_status.clone());
                }

                if ui.button("Log out").clicked {
                    *next_state = State::Market(AccountState::LoggedOut);
                }

                if ui.button("Add item").clicked {}
            },
            AccountState::LoggedOut => {
                ui.heading("Logged out".to_string());

                if ui.button("Log in").clicked {
                    *next_state = State::Login;
                }
            },
        });
    });
}
