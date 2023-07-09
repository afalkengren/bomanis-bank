// impl UserActions {
//     fn as_str(&self) -> &'static str {
//         match self {
//             DEPOSIT => "Deposit Money",
//             WITHDRAW => "Withdraw Money",
//             MAKE_ACCOUNT => "Make a new account",
//         }
//     }
// }

// impl UserActionDeposit for UserAction {
//     self.description = "Deposit money";
// }

pub struct UserAction {
    pub description: &'static str,
}

static UserActionDeposit: UserAction = UserAction {
    description: "Deposit Money",
};

static UserActionWithdraw: UserAction  = UserAction {
    description: "Withdraw Money",
};

static UserActionTransfer: UserAction  = UserAction {
    description: "Transfer Money",
};

static UserActionMakeAccount: UserAction  = UserAction {
    description: "Make new Account",
};


pub static SUPPORTED_USER_ACTIONS: [&UserAction; 4] = [
    &UserActionDeposit, 
    &UserActionWithdraw, 
    &UserActionTransfer, 
    &UserActionMakeAccount
];