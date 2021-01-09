use crate::app::{AccountState, State};

pub struct ProfilePage;

impl ProfilePage {
    pub fn draw(
        ctx: &egui::CtxRef,
        _integration_context: &mut egui::app::IntegrationContext,
        username: &String,
        next_state: &mut State,
        account_status: &mut AccountState,
    ) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Profile");
        });
    }
}
