use super::item::MarketItem;

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum AccountState {
    LoggedOut,
    LoggedIn,
}

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum State {
    Market(AccountState),
    Login,
    Profile(AccountState),
    Item(AccountState, MarketItem),
    AddItem(AccountState),
}

impl Default for State {
    fn default() -> Self { State::Market(AccountState::LoggedOut) }
}

#[derive(PartialEq, Debug, Clone)]
pub enum DeletePromptState {
    Hide,
    Show(String),
}

impl Default for DeletePromptState {
    fn default() -> Self { Self::Hide }
}
