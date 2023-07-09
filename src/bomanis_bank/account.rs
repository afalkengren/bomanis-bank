use crate::bomanis_bank::errors::BalanceError;

enum MoneyAccountType {
    CHECKING,
    SAVINGS,
}

pub struct MoneyAccount {
    linked_bank_account: u32,
    account_type: MoneyAccountType,
    account_number: u32,
    balance: i32,
}

impl MoneyAccount {
    fn get_str(self: &Self) -> &'static str {
        match self.account_type {
            MoneyAccountType::CHECKING => "Checking Account",
            MoneyAccountType::SAVINGS => "Savings Account"
        }
    }

    fn get_account_number(self: &Self) -> u32 {
        return self.account_number;
    }

    fn get_balance(self: &Self) -> i32 {
        return self.balance;
    }

    fn set_balance(self: &mut Self, new_balance: i32) -> Result<i32, BalanceError> {
        self.balance = new_balance;
        return Ok(self.balance)
    }

    fn add_balance(self: &mut Self, amount: i32) -> Result<i32, BalanceError> {
        self.balance += amount;
        return Ok(self.balance)
    }

    fn subtract_balance(self: &mut Self, amount: i32) -> Result<i32, BalanceError> {
        self.balance -= amount;
        return Ok(self.balance)
    }
}

pub struct BankAccount {
    pub account_number: u32,
    pub name: String,
    pub dob: i32,
    accounts: Vec<Box<MoneyAccount>>,
    pub userid: String,
    pub password: String, //make hash
}

impl BankAccount {
    pub fn new(account_number: u32, name: String, dob: i32, userid: String, password: String) -> Self {
        return Self {
            account_number: account_number,
            name: name,
            dob: dob,
            accounts: vec!(),
            userid: userid,
            password: password,
        }
    }
}