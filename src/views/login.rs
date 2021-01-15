use crate::{
    app::{BANK_CONNECTION_ERROR, USER_DATA},
    data::{errors::*, states::*},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BankResponse {
    pub content: String,
    pub value: i32,
}

pub struct LoginPage;

impl LoginPage {
    pub fn get_user_id(username: &str) -> i32 {
        let client = reqwest::blocking::Client::new();
        let mut user_id = 0;

        let response = match client.get("http://157.90.30.90/bankapi/listusers").send() {
            Ok(v) => v,
            Err(_) => {
                *BANK_CONNECTION_ERROR.lock().unwrap() = BankConnectionError::Show(
                    "Could not connect to bank server to get user ID".into(),
                );
                return user_id;
            },
        };

        let users: Vec<String> =
            if let Ok(v) = serde_json::from_str(response.text().unwrap().as_str()) {
                v
            } else {
                Vec::new()
            };

        users.iter().enumerate().into_iter().for_each(|(id, user)| {
            if user == username {
                user_id = (id) as i32;
            }
        });

        USER_DATA.lock().unwrap().id = user_id;

        user_id
    }

    fn get_user_data(
        username: &str,
        password: &str,
        next_state: &mut State,
        show_login_error: &mut LoginError,
    ) {
        let id = Self::get_user_id(username);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                format!("http://157.90.30.90/BankAPI/verifypass/{}/{}", id, password)
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

    pub fn draw(
        ctx: &egui::CtxRef,
        user_data: (&mut String, &mut String),
        password_states: (&mut bool, &mut bool),
        password_colour: &mut egui::Color32,
        next_state: &mut State,
        show_login_error: &mut LoginError,
    ) {
        let (username, password) = user_data;
        let (show_password, remember) = password_states;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login");
            ui.heading("");
            ui.horizontal(|ui| {
                ui.label("Username");
                ui.text_edit_singleline(username);
            });

            ui.horizontal(|ui| {
                ui.label("Password ");
                ui.add(egui::TextEdit::singleline(password).text_color(*password_colour));
                ui.checkbox(show_password, "Show password");
            });

            ui.checkbox(remember, "Remember me");

            if ui.button("Login").clicked && !password.is_empty() && !username.is_empty()
            {
                Self::get_user_data(username, password, next_state, show_login_error);
            }

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
        });
    }
}
