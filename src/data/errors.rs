#[derive(Debug, Clone, PartialEq)]
pub enum BankConnectionError {
    Hide,
    Show(String),
}

impl Default for BankConnectionError {
    fn default() -> Self { BankConnectionError::Hide }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginError {
    Success,
    Fail,
    None,
}

impl Default for LoginError {
    fn default() -> Self { LoginError::None }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarketConnectionError {
    Hide,
    Show(String),
}

impl Default for MarketConnectionError {
    fn default() -> Self { MarketConnectionError::Hide }
}
