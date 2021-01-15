use crate::{
    data::{errors::*, image::*, states::*, user::*, MarketItems},
    views::{AddItemPage, IndexPage, ItemPage, LoginPage, ProfilePage},
};
use std::{sync::Mutex, thread, time::Duration};

lazy_static! {
    pub static ref MARKET_DATA: Mutex<MarketItems> = Mutex::new(MarketItems::new());
    pub static ref USER_DATA: Mutex<User> = Mutex::new(User {
        username: "".into(),
        id: 0,
        balance: 0,
        perms: 0,
    });
    pub static ref MARKET_CONNECTION_ERROR: Mutex<MarketConnectionError> =
        Mutex::new(MarketConnectionError::Hide);
    pub static ref BANK_CONNECTION_ERROR: Mutex<BankConnectionError> =
        Mutex::new(BankConnectionError::Hide);
}

impl USER_DATA {
    pub fn get_clone(&self) -> User { self.lock().unwrap().clone() }
    pub fn get_balance(&self) -> i32 { self.get_clone().balance }
    pub fn get_user_id(&self) -> i32 { self.get_clone().id }
    pub fn get_username(&self) -> String { self.get_clone().username }

    pub fn update(&self, name: &str) {
        #[derive(serde::Deserialize)]
        struct Response {
            balance: i32,
            name: String,
            perm_count: i32,
        };

        let name = if !self.lock().unwrap().username.is_empty() {
            self.lock().unwrap().username.clone()
        } else {
            name.to_string()
        };

        let id = LoginPage::get_user_id(&name);

        let client = reqwest::blocking::Client::new();
        let response: Response = if let Ok(v) = serde_json::from_str(
            match client
                .get(format!("http://157.90.30.90/bankapi/total/{}", id).as_str())
                .send()
            {
                Ok(v) => v.text().unwrap(),
                Err(_) => return,
            }
            .as_str(),
        ) {
            v
        } else {
            return;
        };

        *USER_DATA.lock().unwrap() = User {
            username: response.name.clone(),
            balance: response.balance,
            perms: response.perm_count,
            id,
        };
    }
}

impl MARKET_DATA {
    pub fn update(&self) {
        let client = reqwest::blocking::Client::new();
        let response = match client.get("http://localhost:8000/get").send() {
            Ok(v) => v,
            Err(_) => {
                *MARKET_CONNECTION_ERROR.lock().unwrap() = MarketConnectionError::Show(
                    "Could not connect to market server".into(),
                );
                return;
            },
        };

        *self.lock().unwrap() = if let Ok(mut v) =
            serde_json::from_str::<MarketItems>(response.text().unwrap().as_str())
        {
            v.values_mut().into_iter().for_each(|item| {
                item.image = Image::from_url(&item.item_image_url);
            });

            v
        } else {
            MarketItems::new()
        };

        *MARKET_CONNECTION_ERROR.lock().unwrap() = MarketConnectionError::Hide;
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MarketDashboard {
    pub username: String,
    pub password: String,
    #[serde(skip)]
    pub search_term: String,
    #[serde(skip)]
    pub password_colour: egui::Color32,
    pub show_password: bool,
    pub remember: bool,
    pub state: State,
    #[serde(skip)]
    pub show_login_error: LoginError,
    #[serde(skip)]
    market_update_thread: Option<thread::JoinHandle<()>>,
    #[serde(skip)]
    user_update_thread: Option<thread::JoinHandle<()>>,
}

impl Default for MarketDashboard {
    fn default() -> Self {
        Self {
            username: "".into(),
            password: "".into(),
            search_term: "".into(),
            password_colour: egui::Color32::TRANSPARENT,
            show_password: false,
            remember: false,
            state: State::Market(AccountState::LoggedOut),
            show_login_error: LoginError::None,
            market_update_thread: None,
            user_update_thread: None,
        }
    }
}

impl epi::App for MarketDashboard {
    fn on_exit(&mut self) {
        if self.market_update_thread.is_some() {
            self.market_update_thread = None;
        }

        match self.state.clone() {
            State::Item(acct_status, _)
            | State::Profile(acct_status)
            | State::AddItem(acct_status) => {
                self.state = State::Market(acct_status.clone());

                if acct_status == AccountState::LoggedOut {
                    self.username = "".into();
                    self.password = "".into();
                }
            },
            State::Login => {
                self.state = State::Market(AccountState::LoggedOut);

                self.username = "".into();
                self.password = "".into();
            },
            _ => (),
        }

        if !self.remember {
            *self = Self::default();
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        ctx.request_repaint();

        let MarketDashboard {
            username,
            password,
            search_term,
            show_password,
            remember,
            password_colour,
            state,
            show_login_error,
            market_update_thread,
            user_update_thread,
        } = self;

        if market_update_thread.is_none() {
            *market_update_thread = Some(
                thread::Builder::new()
                    .name("market_update_thread".into())
                    .spawn(move || loop {
                        MARKET_DATA.update();

                        #[cfg(debug_assertions)]
                        println!(
                            "Market data updated at: {}",
                            chrono::Utc::now().format("%A %d/%m/%Y %I:%M:%S %p")
                        );

                        thread::sleep(Duration::new(30, 0));
                    })
                    .unwrap(),
            );
        }

        match state {
            State::Market(acct_status)
            | State::Profile(acct_status)
            | State::Item(acct_status, _)
            | State::AddItem(acct_status) => {
                if *acct_status == AccountState::LoggedIn && user_update_thread.is_none()
                {
                    let username = username.clone();
                    *user_update_thread = Some(
                        thread::Builder::new()
                            .spawn(move || loop {
                                USER_DATA.update(&username);

                                #[cfg(debug_assertions)]
                                println!(
                                    "User data updated at: {}",
                                    chrono::Utc::now().format("%A %d/%m/%Y %I:%M:%S %p")
                                );

                                thread::sleep(Duration::new(30, 0));
                            })
                            .unwrap(),
                    );
                }
            },
            _ => (),
        }

        let mut next_state = state.clone();

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let mut response = ui.heading("CCMarket");

                response = response.interact(egui::Sense::click());

                if response.clicked {
                    match state {
                        State::Item(acct_status, _)
                        | State::Profile(acct_status)
                        | State::AddItem(acct_status) =>
                            next_state = State::Market(acct_status.clone()),
                        State::Login =>
                            next_state = State::Market(AccountState::LoggedOut),
                        _ => (),
                    }
                }
            });
        });

        let mut show_password = show_password;
        let mut remember = remember;

        match state {
            State::Market(acct_status) => IndexPage::draw(
                ctx,
                frame,
                &username,
                search_term,
                acct_status,
                &mut next_state,
            ),
            State::Login => {
                LoginPage::draw(
                    ctx,
                    (username, password),
                    (&mut show_password, &mut remember),
                    password_colour,
                    &mut next_state,
                    show_login_error,
                );
            },
            State::Profile(acct_status) =>
                ProfilePage::draw(ctx, username, &mut next_state, acct_status),
            State::Item(acct_status, item) =>
                ItemPage::draw(ctx, username, &mut next_state, acct_status, item),
            State::AddItem(acct_status) =>
                AddItemPage::draw(ctx, username, &mut next_state, acct_status),
        }

        if *show_password {
            *password_colour = egui::Color32::LIGHT_GRAY;
        } else {
            *password_colour = egui::Color32::TRANSPARENT;
        }

        self.show_password = *show_password;
        self.remember = *remember;

        *state = next_state;
    }

    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str { "CCMarket" }
}
