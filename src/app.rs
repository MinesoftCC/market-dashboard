use crate::{
    data::{errors::*, item::MarketItem, states::*, *},
    views::*,
    THREAD_UPDATE_SYNC,
};
use std::{
    fs::{create_dir, File},
    path::Path,
    sync::atomic::Ordering,
    thread,
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MarketDashboard {
    // general
    pub password: String,
    pub state: State,
    pub username: String,
    #[serde(skip)]
    pub item: MarketItem,
    // --
    // config page specific
    pub market_ip: String,
    pub market_port: String,
    // --
    // background update threads
    #[serde(skip)]
    market_update_thread: Option<thread::JoinHandle<()>>,
    #[serde(skip)]
    user_update_thread: Option<thread::JoinHandle<()>>,
    // --
    // index page specific
    #[serde(skip)]
    pub search_term: String,
    #[serde(skip)]
    pub refresh: bool,
    #[serde(skip)]
    pub delete_prompt_state: DeletePromptState,
    // --
    // login page specific
    #[serde(skip)]
    pub password_colour: egui::Color32,
    #[serde(skip)]
    pub show_password: bool,
    pub remember: bool,
    #[serde(skip)]
    pub show_login_error: LoginError,
    #[serde(skip)]
    pub login_page_state: LoginPageState,
    #[serde(skip)]
    pub confirm_pass: String,
    // --
    // add item page specific
    #[serde(skip)]
    pub item_ratio: u32,
    // --
    // purchase page specific
    #[serde(skip)]
    pub selected_account: String,
    #[serde(skip)]
    pub purchase_amount: u32,
    #[serde(skip)]
    pub show_purchase_error: PurchaseError,
}

impl Default for MarketDashboard {
    fn default() -> Self {
        Self {
            username: String::default(),
            password: String::default(),
            search_term: String::default(),
            confirm_pass: String::default(),
            selected_account: String::default(),
            market_ip: String::default(),
            market_port: String::default(),
            password_colour: egui::Color32::TRANSPARENT,
            show_password: false,
            remember: false,
            refresh: false,
            login_page_state: LoginPageState::Login,
            state: State::Market(AccountState::LoggedOut),
            delete_prompt_state: DeletePromptState::Hide,
            show_login_error: LoginError::None,
            show_purchase_error: PurchaseError::Hide,
            market_update_thread: None,
            user_update_thread: None,
            item: MarketItem::default(),
            item_ratio: 0,
            purchase_amount: 0,
        }
    }
}

impl epi::App for MarketDashboard {
    fn warm_up_enabled(&self) -> bool { true }

    fn on_exit(&mut self) {
        if self.market_update_thread.is_some() {
            *CLOSE_MARKET_THREAD.lock().unwrap() = true;

            self.market_update_thread = None;
        }

        match self.state.clone() {
            State::Item(acct_status, _)
            | State::Profile(acct_status)
            | State::AddItem(acct_status)
            | State::EditItem(acct_status)
            | State::PurchaseItem(acct_status) => {
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
            login_page_state,
            search_term,
            show_password,
            remember,
            refresh,
            password_colour,
            state,
            delete_prompt_state,
            show_login_error,
            show_purchase_error,
            market_update_thread,
            user_update_thread,
            item,
            ..
        } = self;

        match state {
            State::AddItem(_) | State::EditItem(_) | State::PurchaseItem(_) => (),
            _ =>
                if *item != MarketItem::default() {
                    *item = MarketItem::default()
                },
        }

        if market_update_thread.is_none() {
            *market_update_thread = Some(
                thread::Builder::new()
                    .name("market_update_thread".into())
                    .spawn(move || 'a: loop {
                        if *CLOSE_MARKET_THREAD.lock().unwrap() {
                            #[cfg(debug_assertions)]
                            println!("Closed market update thread");
                            break 'a;
                        }

                        MARKET_DATA.update();

                        #[cfg(debug_assertions)]
                        println!(
                            "Market data updated at: {}",
                            chrono::Utc::now().format("%A %d/%m/%Y %I:%M:%S %p")
                        );

                        while !*THREAD_UPDATE_SYNC.read().unwrap() {}
                    })
                    .unwrap(),
            );
        }

        match state {
            State::Market(acct_status)
            | State::Profile(acct_status)
            | State::Item(acct_status, _)
            | State::AddItem(acct_status) => {
                let closed = *CLOSE_USER_THREAD.lock().unwrap();

                if *acct_status == AccountState::LoggedIn && closed {
                    *CLOSE_USER_THREAD.lock().unwrap() = false;
                    let username = username.clone();
                    *user_update_thread = Some(
                        thread::Builder::new()
                            .spawn(move || {
                                *USER_THREAD_COUNT.lock().unwrap().get_mut() += 1;
                                'a: loop {
                                    if *CLOSE_USER_THREAD.lock().unwrap() {
                                        #[cfg(debug_assertions)]
                                        println!("Closed user update thread");

                                        *USER_THREAD_COUNT.lock().unwrap().get_mut() -= 1;

                                        #[cfg(debug_assertions)]
                                        println!(
                                            "Current user thread count: {}",
                                            USER_THREAD_COUNT
                                                .lock()
                                                .unwrap()
                                                .load(Ordering::SeqCst)
                                        );
                                        break 'a;
                                    }

                                    if USER_THREAD_COUNT
                                        .lock()
                                        .unwrap()
                                        .load(Ordering::SeqCst)
                                        > 1
                                    {
                                        #[cfg(debug_assertions)]
                                        println!(
                                            "Closed user update thread. Already have a \
                                             thread running."
                                        );

                                        *USER_THREAD_COUNT.lock().unwrap().get_mut() -= 1;

                                        #[cfg(debug_assertions)]
                                        println!(
                                            "Current user thread count: {}",
                                            USER_THREAD_COUNT
                                                .lock()
                                                .unwrap()
                                                .load(Ordering::SeqCst)
                                        );
                                        break 'a;
                                    }

                                    USER_DATA.update(&username, false);
                                    USER_VEC.update();

                                    #[cfg(debug_assertions)]
                                    println!(
                                        "User data updated at: {}",
                                        chrono::Utc::now()
                                            .format("%A %d/%m/%Y %I:%M:%S %p")
                                    );

                                    while !*THREAD_UPDATE_SYNC.read().unwrap() {}
                                }
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
                        | State::AddItem(acct_status)
                        | State::PurchaseItem(acct_status) =>
                            next_state = State::Market(acct_status.clone()),
                        State::Login => {
                            *login_page_state = LoginPageState::Login;
                            *username = "".into();
                            *password = "".into();

                            next_state = State::Market(AccountState::LoggedOut)
                        },
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
                (
                    &username,
                    search_term,
                    password,
                    &mut self.confirm_pass,
                    &mut self.selected_account,
                ),
                refresh,
                (acct_status, &mut next_state, delete_prompt_state),
                &mut self.item,
            ),
            State::Login => {
                LoginPage::draw(
                    ctx,
                    (username, password, &mut self.confirm_pass),
                    (&mut show_password, &mut remember),
                    password_colour,
                    &mut next_state,
                    &mut self.login_page_state,
                    show_login_error,
                );
            },
            State::Profile(acct_status) =>
                ProfilePage::draw(ctx, username, &mut next_state, acct_status),
            State::Item(acct_status, item) =>
                ItemPage::draw(ctx, username, &mut next_state, acct_status, item),
            State::AddItem(acct_status) => AddItemPage::draw(
                ctx,
                username,
                &mut next_state,
                acct_status,
                item,
                &mut self.item_ratio,
            ),
            State::EditItem(acct_status) => EditItemPage::draw(
                ctx,
                username,
                &mut next_state,
                acct_status,
                item,
                &mut self.item_ratio,
            ),
            State::PurchaseItem(acct_status) => PurchasePage::draw(
                ctx,
                (
                    username,
                    password,
                    &mut self.confirm_pass,
                    &mut self.selected_account,
                ),
                password_colour,
                show_password,
                (&mut next_state, acct_status),
                (item, &mut self.purchase_amount),
                show_purchase_error,
            ),
            State::Config => ConfigPage::draw(
                ctx,
                &mut self.market_ip,
                &mut self.market_port,
                &mut next_state,
            ),
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
        let data_dir_path = Path::new("md-data");
        let instances_file_path = Path::new("md-data/instances.json");

        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();

        if !data_dir_path.exists() {
            if let Err(e) = create_dir(&data_dir_path) {
                eprintln!("Could not create data directory: {}", e);
                return;
            }

            self.state = State::Config;
        }

        if !instances_file_path.exists() {
            if let Err(e) = File::create(instances_file_path) {
                eprintln!(
                    "Could not create instances data file '{}': {}",
                    instances_file_path.display(),
                    e
                );
                return;
            }

            self.state = State::Config;
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str { "CCMarket" }
}
