use std::collections::HashMap;

use crate::bomanis_bank::BankAccount;
use crate::bomanis_bank::actions::SUPPORTED_USER_ACTIONS;
use crate::bomanis_bank::DatabaseHandler;

pub struct Session {
    account_number: Option<u32>,
    database: DatabaseHandler,
}

impl Session {
    pub fn new() -> Self {
        return Session{
            account_number: None,
            database: DatabaseHandler::new(),
        }
    }

    fn handle_input(self: &Self, user_input: u32) {
        match user_input {
            1 => {
                println!("Please enter bank account number:")
            },
            2 => {},
            3 => {},
            4 => {},
            _ => println!("Unknown Option")
        }
    }

    fn login_process(self: &mut Self) -> Result<&BankAccount, std::io::Error> {
        let mut user_id: String = String::new();
        let mut password: String  = String::new();

        let mut loop_i = 0;
        loop {
            println!("User ID:");
            let _ = std::io::stdin().read_line(&mut user_id).unwrap();
            println!("Password:");
            let _ = std::io::stdin().read_line(&mut password).unwrap();
            
            match self.database.findAccountWithUserID(user_id.trim()) {
                Some(account) => return account,
                None => println!("Incorrect details, please try again.")
            }

            if loop_i >= 2 {
                println!("Too many incorrect attempts, please try again...");
            }
            loop_i += 1;
        }
    }

    fn make_account_process(self: &Self) {
        let mut user_id: String = String::new();
        let mut password: String  = String::new();

        loop {
            println!("User ID:");
            let _ = std::io::stdin().read_line(&mut user_id).unwrap();
            if self.database.findAccountWithUserID(user_id.trim()).is_some() {
                println!("That user ID is already in use, please enter another.")
            } else {
                break;
            }
        }

        println!("Password:");
        let _ = std::io::stdin().read_line(&mut password).unwrap();


    }

    pub fn start(self: &mut Self) {
        println!("Welcome to Bomanis Bank!");
        
        match self.login_process() {
            Ok()
        }
        while self.account_number.is_none() {
            println!("Please log in...");
            ;
        }

        if let account: &BankAccount = self.database.getAccountForAccountNumber(self.account_number.unwrap()) {

        }
        println!("Welcome {}!", self.account_number)

        println!("Please choose one of the following options:");
        let max_options: usize = SUPPORTED_USER_ACTIONS.len();
        for i in 0..max_options {
            println!("{}. {}", i+1, SUPPORTED_USER_ACTIONS[i].description)
        }
        println!("{}. Exit", max_options+1);

        loop {
            let mut user_input = String::new();
            let _ = std::io::stdin().read_line(&mut user_input).unwrap();
            match user_input.trim().parse::<u32>() {
                Ok(n) => {
                    if n > (max_options as u32) {
                        break;
                    } else {
                        self.handle_input(n);
                    }
                },
                Err(err) => println!("{}! Invalid input. Try again.", err),
            }
        }
    }
}