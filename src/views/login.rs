use crate::data::{errors::*, states::*, *};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BankResponse {
    pub content: String,
    pub value: i32,
}

pub struct LoginPage;

impl LoginPage {
    fn get_user_data(
        username: &str,
        password: &str,
        next_state: &mut State,
        show_login_error: &mut LoginError,
    ) {
        let id = get_user_id(username);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                format!("{}/verifypass/{}/{}", BANK_API.to_string(), id, password)
                    .as_str(),
            )
            .send();

        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    *BANK_CONNECTION_ERROR.lock().unwrap() =
                        BankConnectionError::Show(format!(
                            "Server responded with a {} code",
                            response.status().as_str()
                        ));
                    return;
                }

                let response: BankResponse =
                    serde_json::from_str(response.text().unwrap().as_str()).unwrap();

                if response.value == 1 {
                    *BANK_CONNECTION_ERROR.lock().unwrap() = BankConnectionError::Hide;
                    *show_login_error = LoginError::Success;
                    *next_state = State::Market(AccountState::LoggedIn);
                } else {
                    *show_login_error = LoginError::Fail;
                }
            },
            Err(error) => {
                *BANK_CONNECTION_ERROR.lock().unwrap() =
                    BankConnectionError::Show(format!(
                        "Could not contact server. Error message:\n{}",
                        format!("{:?}", error).replace(password, "[REDACTED]")
                    ));
                return;
            },
        }

        USER_DATA.lock().unwrap().username = username.into();
        USER_DATA.lock().unwrap().id = id;
        USER_DATA.update(&username, false);
    }

    pub fn create_user(
        username: &str,
        password: &str,
        next_state: &mut State,
        page_state: &mut LoginPageState,
        show_login_error: &mut LoginError,
    ) {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(
                format!(
                    "{}/admin/adduser/{}/{}/{}",
                    BANK_API.to_string(),
                    username,
                    password,
                    ADMIN_PASS.to_string(),
                )
                .as_str(),
            )
            .send();

        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    *BANK_CONNECTION_ERROR.lock().unwrap() =
                        BankConnectionError::Show(format!(
                            "Server responded with a {} code",
                            response.status().as_str()
                        ));
                    return;
                }

                let response: BankResponse =
                    serde_json::from_str(response.text().unwrap().as_str()).unwrap();

                if response.value == 1 {
                    *BANK_CONNECTION_ERROR.lock().unwrap() = BankConnectionError::Hide;
                    *next_state = State::Login;
                    *page_state = LoginPageState::Login;
                    *show_login_error = LoginError::None;
                } else {
                    *show_login_error = LoginError::Fail
                }
            },
            Err(error) => {
                *BANK_CONNECTION_ERROR.lock().unwrap() =
                    BankConnectionError::Show(format!(
                        "Could not contact server. Error message:\n{}",
                        format!("{:?}", error).replace(password, "[REDACTED]")
                    ));
                return;
            },
        }

        USER_DATA.update(&username, false);
    }

    pub fn draw(
        ctx: &egui::CtxRef,
        user_data: (&mut String, &mut String, &mut String),
        password_states: (&mut bool, &mut bool),
        password_colour: &mut egui::Color32,
        next_state: &mut State,
        page_state: &mut LoginPageState,
        show_login_error: &mut LoginError,
    ) {
        let (username, password, confirm_pass) = user_data;
        let (show_password, remember) = password_states;

        egui::CentralPanel::default().show(ctx, |ui| match page_state {
            LoginPageState::Login => {
                ui.heading("Login");
                ui.heading("");
                ui.horizontal(|ui| {
                    ui.label("Username");
                    ui.text_edit_singleline(username);
                });

                ui.horizontal(|ui| {
                    ui.label("Password ");
                    ui.add(
                        egui::TextEdit::singleline(password).text_color(*password_colour),
                    );
                    ui.checkbox(show_password, "Show password");
                });

                ui.checkbox(remember, "Remember me");

                ui.horizontal(|ui| {
                    if ui.button("Login").clicked
                        && !password.is_empty()
                        && !username.is_empty()
                    {
                        Self::get_user_data(
                            username,
                            password,
                            next_state,
                            show_login_error,
                        );
                    }

                    if ui.button("Create a user account").clicked {
                        *username = "".into();
                        *password = "".into();
                        *confirm_pass = "".into();
                        *page_state = LoginPageState::CreateAccount;
                    }

                    if ui.button("Cancel").clicked {
                        *username = "".into();
                        *password = "".into();
                        *page_state = LoginPageState::Login;
                        *next_state = State::Market(AccountState::LoggedOut);
                        *show_login_error = LoginError::None;
                    }
                });

                if password.is_empty() || username.is_empty() {
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "Both fields are required to be filled",
                    );
                }

                if let BankConnectionError::Show(message) =
                    BANK_CONNECTION_ERROR.lock().unwrap().clone()
                {
                    ui.colored_label(egui::Color32::RED, message);
                }

                if let LoginError::Fail = show_login_error {
                    ui.colored_label(
                        egui::Color32::RED,
                        "Password or username was incorrect",
                    );
                }
            },

            LoginPageState::CreateAccount => {
                ui.heading("Create a user account");
                ui.add(
                    egui::Label::new(
                        "Note: this creates a user account on the bank server. \
                         Therefore, this can be used as a tool to create users for the \
                         bank API.",
                    )
                    .text_color(egui::Color32::LIGHT_GRAY)
                    .text_style(egui::TextStyle::Monospace),
                );
                ui.heading("");
                ui.horizontal(|ui| {
                    ui.label("Username");
                    ui.text_edit_singleline(username);
                });

                ui.horizontal(|ui| {
                    ui.label("Password ");
                    ui.add(
                        egui::TextEdit::singleline(password).text_color(*password_colour),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Confirm password ");
                    ui.add(
                        egui::TextEdit::singleline(confirm_pass)
                            .text_color(*password_colour),
                    );
                    ui.checkbox(show_password, "Show password");
                });

                let mut enable = true;

                if username.is_empty() {
                    enable = false;
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "Username field is required to be filled",
                    );
                }

                if !username.chars().all(char::is_alphabetic) {
                    enable = false;
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "Name cannot contain numbers, special letters or punctuation.",
                    );
                }

                if password.is_empty() || confirm_pass.is_empty() {
                    enable = false;
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "Both password fields is required to be filled",
                    );
                }

                if password != confirm_pass {
                    enable = false;
                    ui.colored_label(egui::Color32::RED, "Passwords do not match");
                }

                if USER_VEC
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|uname| uname.to_lowercase())
                    .any(|x| x == username.to_lowercase())
                {
                    enable = false;
                    ui.colored_label(
                        egui::Color32::RED,
                        "A user with that name already exists",
                    );
                }

                ui.horizontal(|ui| {
                    if ui
                        .add(egui::Button::new("Create user").enabled(enable))
                        .clicked
                    {
                        Self::create_user(
                            username,
                            password,
                            next_state,
                            page_state,
                            show_login_error,
                        );
                    }

                    if ui.button("Cancel").clicked {
                        *username = "".into();
                        *password = "".into();
                        *page_state = LoginPageState::Login;
                        *show_login_error = LoginError::None;
                    }
                });

                if let BankConnectionError::Show(message) =
                    BANK_CONNECTION_ERROR.lock().unwrap().clone()
                {
                    ui.colored_label(egui::Color32::RED, message);
                }

                if let LoginError::Fail = show_login_error {
                    ui.colored_label(egui::Color32::RED, "Could not create account");
                }
            },
        });
    }
}
