#[derive(serde::Deserialize, serde::Serialize, Clone)]
enum AccountState {
    LoggedOut,
    LoggedIn,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
enum State {
    Market(AccountState),
    Login,
}

impl Default for State {
    fn default() -> Self { State::Market(AccountState::LoggedOut) }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EguiApp {
    username: String,
    password: String,
    password_colour: egui::color::Srgba,
    show_password: bool,
    #[serde(skip)]
    state: State,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            username: "".into(),
            password: "".into(),
            password_colour: egui::color::TRANSPARENT,
            show_password: false,
            state: State::Market(AccountState::LoggedOut),
        }
    }
}

impl egui::app::App for EguiApp {
    fn name(&self) -> &str { "CCMarket" }

    fn load(&mut self, storage: &dyn egui::app::Storage) {
        *self = egui::app::get_value(storage, egui::app::APP_KEY).unwrap_or_default()
    }

    fn save(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, egui::app::APP_KEY, self);
    }

    fn ui(&mut self, ctx: &egui::CtxRef, integration_context: &mut egui::app::IntegrationContext) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.family_and_size.iter_mut().for_each(|item| {
            let (style, (_, font_size)) = item;

            match style {
                egui::TextStyle::Body => *font_size = 18.0,
                egui::TextStyle::Button => *font_size = 18.0,
                egui::TextStyle::Heading => *font_size = 24.0,
                egui::TextStyle::Small => *font_size = 16.0,
                _ => {},
            }
        });

        ctx.set_fonts(fonts);

        let EguiApp {
            username,
            password,
            show_password,
            password_colour,
            state,
        } = self;
        let mut next_state = state.clone();

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Market");
            });
        });

        let mut show_password = show_password;
        match state {
            State::Market(acct_status) => {
                egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
                    ui.horizontal_wrapped(|ui| match acct_status {
                        AccountState::LoggedIn => {
                            ui.heading(format!("{}", username));

                            if ui.button("Log out").clicked {
                                next_state = State::Market(AccountState::LoggedOut);
                            }
                        },
                        AccountState::LoggedOut => {
                            ui.heading(format!("Logged out"));

                            if ui.button("Log in").clicked {
                                next_state = State::Login;
                            }
                        },
                    })
                });
            },
            State::Login => {
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
                        ui.checkbox(&mut show_password, "Show password");
                    });

                    if ui.button("Login").clicked {
                        if password != "" && username != "" {
                            next_state = State::Market(AccountState::LoggedIn);
                        }
                    }

                    if password == "" || username == "" {
                        ui.colored_label(egui::color::YELLOW, "Both fields are required to be filled");
                    }
                });
            },
        }

        if *show_password {
            *password_colour = egui::color::LIGHT_GRAY;
        } else {
            *password_colour = egui::color::TRANSPARENT;
        }
        self.show_password = *show_password;

        *state = next_state;

        integration_context.output.window_size = Some(ctx.used_size());
    }
}
