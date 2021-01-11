use crate::data::states::*;

pub struct ProfilePage;

impl ProfilePage {
    pub fn draw(ctx: &egui::CtxRef, username: &str, next_state: &mut State, account_status: &mut AccountState) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Profile");
        });
    }
}
