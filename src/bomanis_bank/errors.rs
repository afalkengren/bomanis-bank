pub enum BalanceError {
    NotEnough,
    Locked,
    Unknown,
}

impl BalanceError {
    fn as_str(&self) -> &'static str {
        match self {
            BalanceError::NotEnough => "Not enough balance",
            BalanceError::Locked => "Bank account is locked",
            BalanceError::Unknown => "Unknown error"
        }
    }
}