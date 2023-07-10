use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use rand::Rng;
use chrono::NaiveDate;

use crate::bomanis_bank::account::{BankAccount, BankAccountID, MoneyAccountID, MoneyAccount};

use super::account::MoneyAccountType;

pub struct DatabaseHandler {
    bank_accounts: HashMap<BankAccountID, Rc<RefCell<BankAccount>>>,
    money_accounts: HashMap<MoneyAccountID, BankAccountID>,
}

impl DatabaseHandler {
    pub fn new() -> Self {
        return DatabaseHandler {
            bank_accounts: HashMap::new(),
            money_accounts: HashMap::new(),
        }
    }
    
    // Bank Accounts
    pub fn get_bank_account_with_id_u32(self: &Self, id: BankAccountID) -> Result<Rc<RefCell<BankAccount>>, ()> {
        match self.bank_accounts.get(&id) {
            Some(acc) => return Ok(acc.clone()),
            None => Err(())
        }
    }

    pub fn get_bank_account_with_id_string(self: &Self, id: String) -> Result<Rc<RefCell<BankAccount>>, ()> {
        match id.parse::<u32>() {
            Ok(id) => return self.get_bank_account_with_id_u32(id),
            Err(_) => Err(())
        }
    }

    pub fn check_if_bank_account_id_exists(self: &Self, id: u32) -> bool {
        return self.bank_accounts.contains_key(&id);
    }

    pub fn find_bank_account_with_user_id(self: &Self, user_id: &str) -> Option<Rc<RefCell<BankAccount>>> {
        for account in self.bank_accounts.values() {
            if account.borrow().userid == user_id {
                return Some(account.clone());
            }
        }
        return None;
    }

    pub fn make_new_bank_account(self: &mut Self, name: String, dob: NaiveDate, userid: String, password: String) -> Rc<RefCell<BankAccount>> {
        let mut new_account_number: u32 = rand::thread_rng().gen_range(0..999999);
        while self.check_if_bank_account_id_exists(new_account_number) {
            new_account_number = rand::thread_rng().gen_range(0..999999);
        }
        let new_bank_account = BankAccount::new(
            new_account_number,
            name,
            dob,
            userid,
            password,
        );
        let new_bank_account_ptr = Rc::new(RefCell::new(new_bank_account));
        self.bank_accounts.insert(new_account_number, new_bank_account_ptr.clone());
        return new_bank_account_ptr.clone();
    }

    // Money Account
    pub fn get_money_account_with_id_u32(self: &Self, id: MoneyAccountID) -> Option<Rc<RefCell<MoneyAccount>>> {
        let bank_account_id = self.money_accounts.get(&id)?;
        let bank_account = self.bank_accounts.get(&bank_account_id)?;
        return Some(bank_account.borrow().get_associated_money_account(id)?.clone());
    }

    pub fn get_money_account_with_id_string(self: &Self, id: String) -> Option<Rc<RefCell<MoneyAccount>>> {
        match id.parse::<u32>() {
            Ok(id) => self.get_money_account_with_id_u32(id),
            Err(_) => None,
        }
    }

    pub fn get_money_accounts_with_ids(self: &Self, ids: Vec<MoneyAccountID>) -> Vec<Rc<RefCell<MoneyAccount>>> {
        let mut accs: Vec<Rc<RefCell<MoneyAccount>>> = vec![];
        for id in ids {
            if let Some(acc) = self.get_money_account_with_id_u32(id) {
                accs.push(acc.clone());
            } else {
                //panic!("Unexpected error retrieving account!");
            }
        }
        return accs;
    }

    pub fn check_if_money_account_id_exists(self: &Self, id: u32) -> bool {
        return self.money_accounts.contains_key(&id);
    }

    pub fn make_new_money_account(self: &mut Self, ptr_bank_account: Rc<RefCell<BankAccount>>, name: String, acc_type: MoneyAccountType) -> Rc<RefCell<MoneyAccount>> {
        let mut new_account_number: u32 = rand::thread_rng().gen_range(0..999999);
        while self.check_if_money_account_id_exists(new_account_number) {
            new_account_number = rand::thread_rng().gen_range(0..999999);
        }

        let mut bank_account = ptr_bank_account.borrow_mut();
        let new_acc = MoneyAccount { 
            linked_bank_account: Rc::downgrade(&ptr_bank_account), 
            account_type: acc_type, 
            id: new_account_number, 
            name: name, 
            balance: 0, 
            can_overdraft: false
        };
        let ptr_new_acc = Rc::new(RefCell::new(new_acc));
        bank_account.accounts.push(ptr_new_acc.clone());
        bank_account.accounts_map.insert(new_account_number, Rc::downgrade(&ptr_new_acc));
        self.money_accounts.insert(new_account_number, bank_account.id);
        return ptr_new_acc;
    }

}