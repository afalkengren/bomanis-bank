pub enum SessionError {
    FailedLogin,
    LockedAccount,
    Unknown,
}

impl SessionError {
    fn as_str(&self) -> &'static str {
        match self {
            Self::FailedLogin => "Login failed too many times!",
            Self::LockedAccount => "Bank account is locked!",
            Self::Unknown => "Unknown error",
        }
    }
}

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