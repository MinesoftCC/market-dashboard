use super::{errors::*, image::*, user::*, MarketItems, *};
use std::sync::{atomic::AtomicU8, Mutex};

lazy_static! {
    pub static ref MARKET_DATA: Mutex<MarketItems> = Mutex::new(MarketItems::new());
    pub static ref USER_DATA: Mutex<User> = Mutex::new(User::default());
    pub static ref MARKET_CONNECTION_ERROR: Mutex<MarketConnectionError> =
        Mutex::new(MarketConnectionError::Hide);
    pub static ref BANK_CONNECTION_ERROR: Mutex<BankConnectionError> =
        Mutex::new(BankConnectionError::Hide);
    pub static ref CLOSE_MARKET_THREAD: Mutex<bool> = Mutex::new(false);
    pub static ref CLOSE_USER_THREAD: Mutex<bool> = Mutex::new(true);
    pub static ref USER_THREAD_COUNT: Mutex<AtomicU8> = Mutex::new(AtomicU8::new(0));
    pub static ref USER_VEC: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref BANK_ADDR: Mutex<String> = Mutex::new("".into());
    pub static ref MARKET_ADDR: Mutex<String> = Mutex::new("".into());
}

impl USER_VEC {
    pub fn update(&self) {
        self.lock().unwrap().clear();

        *self.lock().unwrap() = if let Ok(v) = serde_json::from_str(
            reqwest::blocking::get(
                format!("{}/listusers", BANK_API.to_string()).as_str(),
            )
            .unwrap()
            .text()
            .unwrap()
            .as_str(),
        ) {
            v
        } else {
            Vec::default()
        }
    }
}

impl USER_DATA {
    pub fn get_clone(&self) -> User { self.lock().unwrap().clone() }
    pub fn get_balance(&self) -> i32 { self.get_clone().balance }
    pub fn get_user_id(&self) -> i32 { self.get_clone().id }
    pub fn get_username(&self) -> String { self.get_clone().username }

    pub fn update(&self, name: &str, logout: bool) {
        if logout {
            *self.lock().unwrap() = User::default();
            *CLOSE_USER_THREAD.lock().unwrap() = true;
            return;
        }

        #[derive(serde::Deserialize)]
        struct UserResponse {
            balance: i32,
            name: String,
            perm_count: i32,
        };

        let name = if !self.lock().unwrap().username.is_empty() {
            self.lock().unwrap().username.clone()
        } else {
            name.to_string()
        };

        let id = get_user_id(&name);
        let client = reqwest::blocking::Client::new();
        let user_response: UserResponse = if let Ok(v) = serde_json::from_str(
            match client
                .get(format!("{}/total/{}", BANK_API.to_string(), id).as_str())
                .send()
            {
                Ok(v) => v.text().unwrap(),
                Err(_) => {
                    *CLOSE_USER_THREAD.lock().unwrap() = true;
                    return;
                },
            }
            .as_str(),
        ) {
            v
        } else {
            *CLOSE_USER_THREAD.lock().unwrap() = true;
            return;
        };

        let account_response: UserAccounts = if let Ok(v) = serde_json::from_str(
            match client
                .get(format!("{}/listaccounts/{}", BANK_API.to_string(), id).as_str())
                .send()
            {
                Ok(v) => v.text().unwrap(),
                Err(_) => {
                    *CLOSE_USER_THREAD.lock().unwrap() = true;
                    return;
                },
            }
            .as_str(),
        ) {
            v
        } else {
            *CLOSE_USER_THREAD.lock().unwrap() = true;
            return;
        };

        *USER_DATA.lock().unwrap() = User {
            username: user_response.name.clone(),
            balance: user_response.balance,
            perms: user_response.perm_count,
            accounts: account_response,
            id,
        };
    }
}

impl MARKET_DATA {
    pub fn update(&self) {
        let client = reqwest::blocking::Client::new();
        let response = match client
            .get(format!("{}/get", MARKET_API.to_string()).as_str())
            .send()
        {
            Ok(v) => v,
            Err(_) => {
                *MARKET_CONNECTION_ERROR.lock().unwrap() = MarketConnectionError::Show(
                    "Could not connect to market server".into(),
                );
                return;
            },
        };

        if let Ok(v) =
            serde_json::from_str::<MarketItems>(response.text().unwrap().as_str())
        {
            v.into_iter().for_each(|(key, value)| {
                let mut self_lock = self.lock().unwrap();

                if self_lock.contains_key(&key) {
                    let item = self_lock.get(&key).unwrap();
                    if (item.image.pixels.is_empty()
                        && !value.image.pixels.is_empty()
                        && item.image.pixels != value.image.pixels)
                        || *item != value
                    {
                        let _ = self_lock.insert(key, value);
                    }
                } else {
                    self_lock.insert(key, value);
                }
            });
        }

        self.lock().unwrap().retain(|_, item| !item.deleted);

        self.lock().unwrap().iter_mut().for_each(|(_, item)| {
            if item.image.pixels.is_empty() {
                #[cfg(debug_assertions)]
                println!(
                    "Updating image for {} uploaded by user ID {}",
                    item.display_name, item.poster_id
                );

                item.image = Image::from_url(&item.item_image_url);
            }
        });

        *MARKET_CONNECTION_ERROR.lock().unwrap() = MarketConnectionError::Hide;
    }
}
