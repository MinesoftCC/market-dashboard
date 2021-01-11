use super::item::MarketItem;

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
    ItemPage(AccountState, MarketItem),
}

impl Default for State {
    fn default() -> Self { State::Market(AccountState::LoggedOut) }
}
