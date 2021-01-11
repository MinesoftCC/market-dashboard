use crate::data::{item::*, states::*};

pub struct ItemPage;

impl ItemPage {
    pub fn draw(
        ctx: &egui::CtxRef,
        username: &str,
        next_state: &mut State,
        account_status: &mut AccountState,
        item: &MarketItem,
    ) {
        super::draw_sidebar(ctx, username, next_state, account_status);

        let item = item.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(item.display_name);
        });
    }
}
