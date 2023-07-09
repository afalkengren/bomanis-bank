use std::collections::HashMap;
use rand::Rng;
use crate::bomanis_bank::account::BankAccount;

pub struct DatabaseHandler {
    available_accounts: HashMap<u32, BankAccount>,
}

impl DatabaseHandler {
    pub fn new() -> Self {
        return DatabaseHandler {
            available_accounts: HashMap::new()
        }
    }
    
    pub fn getAccountForAccountNumber(self: &Self, account_number: u32) -> Result<&BankAccount> {
        return self.available_accounts.get(&account_number);
    }

    pub fn getAccountForAccountNumberStr(self: &Self, account_number: String) -> Option<&BankAccount> {
        match account_number.parse::<u32>() {
            Ok(account_number) => self.available_accounts.get(&account_number),
            Err(_) => None
        }
    }

    pub fn accountForAccountNumberExists(self: &Self, account_number: u32) -> bool {
        return self.available_accounts.contains_key(&account_number);
    }

    pub fn findAccountWithUserID(self: &Self, user_id: &str) -> Option<&BankAccount> {
        for account in self.available_accounts.values() {
            if account.userid == user_id {
                return Some(account);
            }
        }
        return None;
    }

    pub fn makeNewAccount(self: &mut Self, name: String, dob: i32, userid: String, password: String) -> Option<&BankAccount> {
        let mut new_account_number: u32 = rand::thread_rng().gen_range(0..999999);
        while self.accountForAccountNumberExists(new_account_number) {
            new_account_number = rand::thread_rng().gen_range(0..999999);
        }
        let new_bank_account = BankAccount::new(
            new_account_number,
            name,
            dob,
            userid,
            password,
        );
        self.available_accounts.insert(new_account_number, new_bank_account);
        return self.available_accounts.get(&new_account_number)
    }
}