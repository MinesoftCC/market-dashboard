use crate::views::{IndexPage, LoginPage, ProfilePage};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum AccountState {
    LoggedOut,
    LoggedIn,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum State {
    Market(AccountState),
    Login,
    Profile(AccountState),
}

impl Default for State {
    fn default() -> Self { State::Market(AccountState::LoggedOut) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BankConnectionError {
    Hide,
    Show(String),
}

impl Default for BankConnectionError {
    fn default() -> Self { BankConnectionError::Hide }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginError {
    Success,
    Fail,
    None,
}

impl Default for LoginError {
    fn default() -> Self { LoginError::None }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MarketDashboard {
    pub username: String,
    pub password: String,
    #[serde(skip)]
    pub password_colour: egui::color::Color32,
    pub show_password: bool,
    pub remember: bool,
    pub state: State,
    #[serde(skip)]
    pub show_bank_connection_error: BankConnectionError,
    #[serde(skip)]
    pub show_login_error: LoginError,
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
        }
    }
}

impl epi::App for MarketDashboard {
    fn name(&self) -> &str { "CCMarket" }

    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        if !self.remember {
            *self = Self::default();
        }

        epi::set_value(storage, epi::APP_KEY, self);
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
        } = self;

        let mut next_state = state.clone();

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let mut response = ui.heading("Market");

                response = response.interact(egui::Sense::click());

                if response.clicked {
                    match state {
                        State::Profile(acct_status) => next_state = State::Market(acct_status.clone()),
                        State::Login => next_state = State::Market(AccountState::LoggedOut),
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
}
