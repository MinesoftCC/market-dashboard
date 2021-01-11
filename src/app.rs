use crate::{
    data::{errors::*, states::*, MarketItems},
    views::{IndexPage, ItemPage, LoginPage, ProfilePage},
};
use chrono::prelude::*;
use std::{sync::Mutex, thread, time::Duration};

lazy_static! {
    pub static ref MARKET_DATA: Mutex<MarketItems> = Mutex::new(MarketItems::new());
    pub static ref MARKET_CONNECTION_ERROR: Mutex<MarketConnectionError> = Mutex::new(MarketConnectionError::Hide);
}

impl MARKET_DATA {
    pub fn update(&self) {
        let client = reqwest::blocking::Client::new();
        let response = match client.get("http://localhost:8000/get").send() {
            Ok(v) => {
                *MARKET_CONNECTION_ERROR.lock().unwrap() = MarketConnectionError::Hide;
                v
            },
            Err(_) => {
                *MARKET_CONNECTION_ERROR.lock().unwrap() =
                    MarketConnectionError::Show("Could not connect to market server".into());
                return;
            },
        };

        let market_items = if let Ok(v) = serde_json::from_str(response.text().unwrap().as_str()) {
            v
        } else {
            MarketItems::new()
        };

        *self.lock().unwrap() = market_items;
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MarketDashboard {
    pub username: String,
    pub password: String,
    #[serde(skip)]
    pub password_colour: egui::Color32,
    pub show_password: bool,
    pub remember: bool,
    pub state: State,
    #[serde(skip)]
    pub show_bank_connection_error: BankConnectionError,
    #[serde(skip)]
    pub show_login_error: LoginError,
    #[serde(skip)]
    market_update_thread: Option<thread::JoinHandle<()>>,
}

impl Default for MarketDashboard {
    fn default() -> Self {
        Self {
            username: "".into(),
            password: "".into(),
            password_colour: egui::color::Color32::TRANSPARENT,
            show_password: false,
            remember: false,
            state: State::Market(AccountState::LoggedOut),
            show_bank_connection_error: BankConnectionError::Hide,
            show_login_error: LoginError::None,
            market_update_thread: None,
        }
    }
}

impl epi::App for MarketDashboard {
    fn on_exit(&mut self) {
        if self.market_update_thread.is_some() {
            self.market_update_thread = None;
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let MarketDashboard {
            username,
            password,
            show_password,
            remember,
            password_colour,
            state,
            show_bank_connection_error,
            show_login_error,
            market_update_thread,
        } = self;

        if market_update_thread.is_none() {
            *market_update_thread = Some(thread::spawn(|| loop {
                MARKET_DATA.update();
                println!("Market data updated at: {}", Utc::now().time());
                thread::sleep(Duration::new(60, 0));
            }));
        }

        let mut next_state = state.clone();

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let mut response = ui.heading("Market");

                response = response.interact(egui::Sense::click());

                if response.clicked {
                    match state {
                        State::Profile(acct_status) => next_state = State::Market(acct_status.clone()),
                        State::Login => next_state = State::Market(AccountState::LoggedOut),
                        State::ItemPage(acct_status, _) => next_state = State::Market(acct_status.clone()),
                        _ => (),
                    }
                }
            });
        });

        let mut show_password = show_password;
        let mut remember = remember;

        match state {
            State::Market(acct_status) => IndexPage::draw(ctx, &username, acct_status, &mut next_state),
            State::Login => {
                LoginPage::draw(
                    ctx,
                    frame,
                    (username, password),
                    (&mut show_password, &mut remember),
                    password_colour,
                    &mut next_state,
                    (show_bank_connection_error, show_login_error),
                );
            },
            State::Profile(acct_status) => ProfilePage::draw(ctx, username, &mut next_state, acct_status),
            State::ItemPage(acct_status, item) => ItemPage::draw(ctx, username, &mut next_state, acct_status, item),
        }

        if *show_password {
            *password_colour = egui::color::Color32::LIGHT_GRAY;
        } else {
            *password_colour = egui::color::Color32::TRANSPARENT;
        }

        self.show_password = *show_password;
        self.remember = *remember;
        self.show_bank_connection_error = show_bank_connection_error.clone();

        *state = next_state;
    }

    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        if !self.remember {
            *self = Self::default();
        }

        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str { "CCMarket" }
}
