use crate::data::{errors::*, states::*};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BankResponse {
    pub content: String,
    pub value: i32,
}

pub struct LoginPage;

impl LoginPage {
    fn get_user_id(
        username: &str,
        show_bank_connection_error: &mut BankConnectionError,
    ) -> i32 {
        let client = reqwest::blocking::Client::new();
        let mut user_id = 0;

        let response =
            match client.get("http://157.90.30.90/bankapi/listusers").send() {
                Ok(v) => v,
                Err(_) => {
                    *show_bank_connection_error = BankConnectionError::Show(
                        "Could not connect to bank server to get user ID"
                            .into(),
                    );
                    return user_id;
                },
            };

        let users: Vec<String> = if let Ok(v) =
            serde_json::from_str(response.text().unwrap().as_str())
        {
            v
        } else {
            Vec::new()
        };

        users.iter().enumerate().into_iter().for_each(|(id, user)| {
            if user == username {
                user_id = (id) as i32;
            }
        });

        user_id
    }

    fn get_user_data(
        username: &str,
        password: &str,
        next_state: &mut State,
        show_bank_connection_error: &mut BankConnectionError,
        show_login_error: &mut LoginError,
    ) {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(
                format!(
                    "http://157.90.30.90/BankAPI/verifypass/{}/{}",
                    Self::get_user_id(username, show_bank_connection_error),
                    password
                )
                .as_str(),
            )
            .send();

        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    *show_bank_connection_error =
                        BankConnectionError::Show(format!(
                            "Server responded with a {} code",
                            response.status().as_str()
                        ));
                    return;
                }

                let response: BankResponse =
                    serde_json::from_str(response.text().unwrap().as_str())
                        .unwrap();

                if response.value == 1 {
                    *show_bank_connection_error = BankConnectionError::Hide;
                    *show_login_error = LoginError::Success;
                    *next_state = State::Market(AccountState::LoggedIn);
                } else {
                    *show_login_error = LoginError::Fail;
                }
            },
            Err(error) => {
                *show_bank_connection_error =
                    BankConnectionError::Show(format!(
                        "Could not contact server. Error message:\n{}",
                        format!("{:?}", error).replace(password, "[REDACTED]")
                    ));
            },
        }
    }

    pub fn draw(
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        user_data: (&mut String, &mut String),
        password_states: (&mut bool, &mut bool),
        password_colour: &mut egui::Color32,
        next_state: &mut State,
        error_states: (&mut BankConnectionError, &mut LoginError),
    ) {
        let (username, password) = user_data;
        let (show_password, remember) = password_states;
        let (show_bank_connection_error, show_login_error) = error_states;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login");
            ui.heading("");
            ui.horizontal(|ui| {
                ui.label("Username");
                ui.text_edit_singleline(username);
            });

            ui.horizontal(|ui| {
                ui.label("Password ");
                ui.add(
                    egui::TextEdit::singleline(password)
                        .text_color(*password_colour),
                );
                ui.checkbox(show_password, "Show password");
            });

            ui.checkbox(remember, "Remember me");

            if ui.button("Login").clicked
                && !password.is_empty()
                && !username.is_empty()
            {
                Self::get_user_data(
                    username,
                    password,
                    next_state,
                    show_bank_connection_error,
                    show_login_error,
                );
            }

            if password.is_empty() || username.is_empty() {
                ui.colored_label(
                    egui::Color32::YELLOW,
                    "Both fields are required to be filled",
                );
            }

            if let BankConnectionError::Show(message) =
                show_bank_connection_error
            {
                ui.colored_label(egui::Color32::RED, message.clone());
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
