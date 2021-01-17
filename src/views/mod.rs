mod add_item;
mod edit_item;
mod index;
mod item_page;
mod login;
mod profile;
mod purchase_item;

pub use add_item::*;
pub use edit_item::*;
pub use index::*;
pub use item_page::*;
pub use login::*;
pub use profile::*;
pub use purchase_item::*;

use crate::{
    app::{USER_DATA, USER_VEC},
    data::states::*,
};

fn draw_sidebar(
    ctx: &egui::CtxRef,
    username: &str,
    next_state: &mut State,
    account_status: &mut AccountState,
) {
    egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| match account_status {
        AccountState::LoggedIn => {
            ui.horizontal_wrapped(|ui| {
                let mut response = ui.heading(username.to_string()).on_hover_ui(|ui| {
                    ui.label("Click on your username to go to your profile");
                });

                response = response.interact(egui::Sense::click());

                if response.clicked {
                    *next_state = State::Profile(account_status.clone());
                }

                if ui.button("Log out").clicked {
                    *next_state = State::Market(AccountState::LoggedOut);
                    USER_DATA.update("", true);
                }

                if ui.button("Add item").clicked {
                    *next_state = State::AddItem(account_status.clone());
                }
            });

            ui.label(format!("Current balance: {}", USER_DATA.get_balance()));
        },
        AccountState::LoggedOut => {
            ui.horizontal_wrapped(|ui| {
                ui.heading("Logged out".to_string());

                if ui.button("Log in").clicked {
                    *next_state = State::Login;
                }
            });
        },
    });
}

pub fn get_name_from_id(id: u16) -> String {
    let user_vec = USER_VEC.lock().unwrap();

    if id as usize > user_vec.len() {
        "Invalid user".to_string()
    } else if id == USER_DATA.get_user_id() as u16 {
        format!("{} (You)", user_vec[id as usize])
    } else {
        user_vec[id as usize].to_string()
    }
}
