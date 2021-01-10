use crate::app::{AccountState, State};

pub struct IndexPage;

impl IndexPage {
    pub fn draw(ctx: &egui::CtxRef, username: &str, account_status: &mut AccountState, next_state: &mut State) {
        super::draw_sidebar(ctx, username, next_state, account_status);
    }
}
