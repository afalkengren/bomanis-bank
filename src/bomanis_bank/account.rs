use chrono::NaiveDate;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};

use crate::bomanis_bank::errors::BalanceError;

// Type Aliases
pub type MoneyAccountID = u32;
pub type BankAccountID = u32;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MoneyAccountType {
    Checking,
    Savings,
    Length,
}
pub struct MoneyAccount {
    pub linked_bank_account: Weak<RefCell<BankAccount>>,
    pub account_type: MoneyAccountType,
    pub id: MoneyAccountID,
    pub name: String,
    pub balance: i32,
    pub can_overdraft: bool,
}

impl MoneyAccount {
    pub fn get_str(self: &Self) -> &'static str {
        match self.account_type {
            MoneyAccountType::Checking => "Checkings Account",
            MoneyAccountType::Savings => "Savings Account",
            MoneyAccountType::Length => "NONE",
        }
    }

    pub fn get_account_number(self: &Self) -> u32 {
        return self.id;
    }

    pub fn get_balance(self: &Self) -> i32 {
        return self.balance;
    }

    pub fn set_balance(self: &mut Self, new_balance: i32) -> Result<i32, BalanceError> {
        self.balance = new_balance;
        return Ok(self.balance)
    }

    pub fn add_balance(self: &mut Self, amount: i32) -> Result<i32, BalanceError> {
        self.balance += amount;
        return Ok(self.balance)
    }

    pub fn subtract_balance(self: &mut Self, amount: i32) -> Result<i32, BalanceError> {
        if self.balance - amount < 0 && !self.can_overdraft {
            return Err(BalanceError::NotEnough);
        } else {
            self.balance -= amount;
            return Ok(self.balance)
        }
    }

    pub fn get_bank_account(self: &Self) -> Rc<RefCell<BankAccount>> {
        // should always exist, or we have a problem...
        return self.linked_bank_account.upgrade().unwrap();
    }
}

pub enum UserActions {
    Deposit,
    Withdraw,
    ShowBalance,
    CreateSavings,
    CreateCheckings,
    Logout,
    Exit,
    Length,
}

impl UserActions {
    pub fn description(self: &Self) -> &'static str {
        match self {
            UserActions::Deposit => "Deposit money",
            UserActions::Withdraw => "Withdraw money",
            UserActions::ShowBalance => "Show balance",
            UserActions::CreateSavings => "Create savings account",
            UserActions::CreateCheckings => "Create checkings account",
            UserActions::Logout => "Logout",
            UserActions::Exit => "Exit",
            UserActions::Length => "NONE",
        }
    }    
}

pub struct BankAccount {
    pub id: BankAccountID,
    pub name: String,
    pub dob: NaiveDate,
    pub accounts: Vec<Rc<RefCell<MoneyAccount>>>,
    pub accounts_map: HashMap<MoneyAccountID, Weak<RefCell<MoneyAccount>>>,
    pub userid: String,
    pub password: String, //TODO make hash
}

impl BankAccount {
    pub fn new(account_number: u32, name: String, dob: NaiveDate, userid: String, password: String) -> Self {
        return Self {
            id: account_number as BankAccountID,
            name: name,
            dob: dob,
            accounts: vec!(),
            accounts_map: HashMap::new(),
            userid: userid,
            password: password,
        }
    }

    pub fn get_associated_money_account(self: &Self, id: MoneyAccountID) -> Option<Rc<RefCell<MoneyAccount>>> {
        return Some(self.accounts_map.get(&id)?.upgrade()?.clone());
    }
}