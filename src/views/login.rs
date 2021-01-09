use crate::app::{AccountState, BankConnectionError, LoginError, State};
use reqwest;

#[derive(serde::Deserialize, serde::Serialize)]
struct Response {
    pub content: String,
    pub value: i32,
}

pub struct LoginPage;

impl LoginPage {
    #[cfg(target_arch = "wasm32")]
    fn get_user_data(
        username: &String,
        password: &String,
        next_state: &mut State,
        show_bank_connection_error: &mut BankConnectionError,
        _show_login_error: &mut LoginError,
    ) {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{Request, RequestInit, RequestMode, Response};
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_user_id(username: &String) -> i32 {
        let client = reqwest::blocking::Client::new();
        let mut user_id = 0;

        let users: Vec<String> = if let Ok(v) = serde_json::from_str(
            client
                .get("http://157.90.30.90/bankapi/listusers")
                .send()
                .unwrap()
                .text()
                .unwrap()
                .as_str(),
        ) {
            v
        } else {
            Vec::new()
        };

        users.iter().enumerate().into_iter().for_each(|(id, user)| {
            if user == username {
                user_id = (id) as i32;
            }
        });

        return user_id;
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_user_data(
        username: &String,
        password: &String,
        next_state: &mut State,
        show_bank_connection_error: &mut BankConnectionError,
        _show_login_error: &mut LoginError,
    ) {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(
                format!(
                    "http://157.90.30.90/bankapi/vpass/{}/{}",
                    Self::get_user_id(username),
                    password
                )
                .as_str(),
            )
            .send();

        match response {
            Ok(response) => {
                if !response.status().is_success() {
                    *show_bank_connection_error = BankConnectionError::Show(format!(
                        "Server responded with a {} code",
                        response.status().as_str()
                    ));
                    return;
                }

                *show_bank_connection_error = BankConnectionError::Hide;
                *next_state = State::Market(AccountState::LoggedIn);
            },
            Err(error) => {
                *show_bank_connection_error = BankConnectionError::Show(format!(
                    "Could not contact server. Error message:\n{}",
                    format!("{:?}", error).replace(password, "[REDACTED]")
                ));
            },
        }
    }

    pub fn draw(
        ctx: &egui::CtxRef,
        _integration_context: &mut egui::app::IntegrationContext,
        username: &mut String,
        password: &mut String,
        show_password: &mut bool,
        remember: &mut bool,
        password_colour: &mut egui::color::Srgba,
        next_state: &mut State,
        show_bank_connection_error: &mut BankConnectionError,
        show_login_error: &mut LoginError,
    ) {
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

            if ui.button("Login").clicked && !password.is_empty() && !username.is_empty() {
                Self::get_user_data(
                    username,
                    password,
                    next_state,
                    show_bank_connection_error,
                    show_login_error,
                );
            }

            if password.is_empty() || username.is_empty() {
                ui.colored_label(egui::color::YELLOW, "Both fields are required to be filled");
            }

            if let BankConnectionError::Show(message) = show_bank_connection_error {
                ui.colored_label(egui::color::RED, message.clone());
            }
        });
    }
}
