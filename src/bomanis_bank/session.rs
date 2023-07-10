use std::io::Write;
use std::ptr;
use std::rc::Rc;
use std::cell::RefCell;
use regex::Regex;
use chrono::NaiveDate;

use crate::bomanis_bank::account::{
    BankAccount,
    MoneyAccount,
    MoneyAccountType,
    UserActions
};
use crate::bomanis_bank::DatabaseHandler;
use crate::bomanis_bank::errors;

pub struct Session {
    bank_account: Option<Rc<RefCell<BankAccount>>>, // ownership of bank account
    database: DatabaseHandler,
}

impl Session {
    pub fn new() -> Self {
        return Session{
            bank_account: None,
            database: DatabaseHandler::new(),
        }
    }

    // login process will return true if successful.
    // Takes ownership of the user's bank account from the hashmap.
    fn login_process(self: &mut Self) -> bool {
        let mut user_id: String = String::new();
        let mut password: String  = String::new();

        let mut max_retry_loop_i = 3; // maximum times we can retry login before exiting.
        loop {
            max_retry_loop_i -= 1;
            print!("User ID: ");
            let _ = std::io::stdout().flush();
            user_id = get_user_input_as_str();
            print!("Password: ");
            let _ = std::io::stdout().flush();
            password = get_user_input_as_str();
            
            match self.database.find_bank_account_with_user_id(user_id.trim()) {
                Some(account) => {
                    self.bank_account = Some(account.clone()); // take ownership of bank account
                    return true;
                }
                None => println!("Incorrect details, please try again.")
            }

            if max_retry_loop_i <= 0 {
                println!("Too many incorrect attempts, please try again later...");
                return false;
            }
        }
    }

    fn make_account_process(self: &mut Self) -> bool {
        println!("Thank you for making an account at Bomanis Bank!");
        println!("Please enter the following details to get started...");
        let mut user_id: String;
        loop {
            print!("User ID: ");
            let _ = std::io::stdout().flush();
            user_id = get_user_input_as_str();
            if self.database.find_bank_account_with_user_id(&user_id).is_some() {
                println!("That user ID is already in use, please enter another.")
            } else {
                break;
            }
        }

        print!("Password: ");
        let _ = std::io::stdout().flush();
        let password = get_user_input_as_str();

        print!("Name: ");
        let _ = std::io::stdout().flush();
        let name = get_user_input_as_str();

        let dob_re = Regex::new("^([0-9]{4}-[0-9]{2}-[0-9]{2})$").unwrap();
        let mut dob_str: String;
        let dob: NaiveDate;
        loop {
            print!("Date of Birth (YYYY-MM-DD): ");
            let _ = std::io::stdout().flush();
            dob_str = get_user_input_as_str();
            if let Some(captures) = dob_re.captures(&dob_str) {
                let captured_dob = captures.get(1).unwrap().as_str();
                if let Ok(date) = NaiveDate::parse_from_str(captured_dob, "%Y-%m-%d") {
                    dob = date;
                    break;
                } else {
                    println!("\nInvalid DOB! Please enter a correct date.");
                }
            } else {
                println!("\nInvalid DOB! Please enter in the format specified.");
            }
        }
        let new_bank_account = self.database.make_new_bank_account(name, dob, user_id, password);
        println!("Account successfully created! Your new account number is {}.", new_bank_account.borrow().id);
        self.bank_account = Some(new_bank_account);
        return true;
    }

    pub fn start(self: &mut Self) -> Result<(), errors::SessionError> {
        println!("Welcome to Bomanis Bank!");
        
        loop {
            loop {
                println!("\r\nChoose the following options:");
                println!("1. Login");
                println!("2. Make a new account");
                println!("0. Exit");
                
                print!("Enter a number: ");
                let _ = std::io::stdout().flush();
                match get_user_input_as_i32() {
                    1 => { // Login
                        if self.login_process() {
                            break;
                        } else {
                            return Err(errors::SessionError::FailedLogin);
                        }
                    },
                    2 => { // Make Account
                        if self.make_account_process() {
                            break;
                        } else {
                            return Err(errors::SessionError::Unknown);
                        }
                    },
                    0 => return Ok(()),
                    _ => println!("Invalid input. Please try again."),
                }
            }

            println!("\r\nWelcome {}!", self.bank_account_for_session().borrow().name);

            // Looping user input dialogue
            loop {
                println!("Please choose one of the following options:");
                let supported_actions: Vec<UserActions> = self.get_valid_actions(self.bank_account_for_session());
                for (i, actions) in supported_actions.iter().enumerate() {
                    println!("{}. {}", i+1, actions.description())
                }

                print!("Enter a number: ");
                let _ = std::io::stdout().flush();
                let user_input = (get_user_input_as_i32() - 1) as usize; // subtract one for zero-indexing
                if user_input < supported_actions.len() {
                    match supported_actions[user_input] {
                        UserActions::CreateSavings => self.action_create_money_account(MoneyAccountType::Savings),
                        UserActions::CreateCheckings => self.action_create_money_account(MoneyAccountType::Checking),
                        UserActions::Deposit => self.action_deposit(),
                        UserActions::Withdraw => self.action_withdraw(),
                        UserActions::Logout => {
                            self.bank_account = None;
                            break;
                        },
                        UserActions::Exit => break,
                        _ => return Err(errors::SessionError::Unknown),
                    }
                } else {
                    println!("Invalid input. Please try again.");
                }
            }
        }
    }

    // Returns the unwrapped BankAccount.
    // Should always be used after login, and should be valid.
    fn bank_account_for_session(self: &Self) -> Rc<RefCell<BankAccount>> {
        match &self.bank_account {
            Some(acc) => return acc.clone(),
            None => panic!("Unexpected error retrieving bank account!")
        }
    }

    fn action_create_money_account(self: &mut Self, acc_type: MoneyAccountType) {
        print!("Please enter your name for this account: ");
        let _ = std::io::stdout().flush();
        let name = get_user_input_as_str();
        let ptr_bank_account = self.bank_account_for_session();
        let ptr_new_acc = self.database.make_new_money_account(ptr_bank_account, name, acc_type);
        let new_acc = ptr_new_acc.borrow();
        println!("Thank you, your new {}, {} ({}) has been created!", new_acc.get_str(), new_acc.name, new_acc.id);
    }
    
    fn action_deposit(self: &Self) {
        println!("Deposit to which account?");
        let mut user_selectable_accs: Vec<Rc<RefCell<MoneyAccount>>> = vec!();
        for ptr_acc in &self.bank_account_for_session().borrow().accounts {
            user_selectable_accs.push(ptr_acc.clone());
            println!("{}. {} ({})", user_selectable_accs.len(), ptr_acc.borrow().name, ptr_acc.borrow().id);
        }
        println!("0. Exit");
        loop {
            print!("Enter a number: ");
            let _ = std::io::stdout().flush();
            let user_input = get_user_input_as_i32();
            if user_input == 0 {
                return;
            } else if user_input <= (user_selectable_accs.len() as i32) {
                let selected_acc_index = (user_input - 1) as usize; // subtract one for zero-indexing
                let ptr_money_account = user_selectable_accs[selected_acc_index].clone();
                let mut money_account = ptr_money_account.borrow_mut();
                println!("Depositing into {}...", money_account.name);
                print!("Enter deposit value: ");
                let _ = std::io::stdout().flush();
                let _ = money_account.add_balance(get_user_input_as_i32());
                println!("Deposit successful. Your current balance is ${}", money_account.balance);
                return;
            } else {
                println!("Invalid input. Please try again.");
            }
        }
    }

    fn action_withdraw(self: &Self) {
        println!("Withdraw from which account?");
        
        let mut user_selectable_accs: Vec<Rc<RefCell<MoneyAccount>>> = vec!();
        for ptr_acc in &self.bank_account_for_session().borrow().accounts {
            user_selectable_accs.push(ptr_acc.clone());
            println!("{}. {} ({})", user_selectable_accs.len(), ptr_acc.borrow().name, ptr_acc.borrow().id);
        }
        println!("0. Exit");

        loop {
            print!("Enter a number: ");
            let _ = std::io::stdout().flush();
            let user_input = get_user_input_as_i32();
            if user_input == 0 {
                return;
            } else if user_input <= (user_selectable_accs.len() as i32) {
                let selected_acc_index = (user_input - 1) as usize; // subtract one for zero-indexing
                let ptr_money_account = user_selectable_accs[selected_acc_index].clone();
                let mut money_account = ptr_money_account.borrow_mut();
                println!("Withdrawing from {}...", money_account.name);
                print!("Enter withdraw value: ");
                let _ = std::io::stdout().flush();
                match money_account.subtract_balance(get_user_input_as_i32()) {
                    Ok(new_balance) => println!("Withdraw successful. Your current balance is ${}", new_balance),
                    Err(err) => match err {
                        errors::BalanceError::NotEnough => println!("Sorry, you do not have enough balance for this transaction."),
                        _ => println!("Sorry, something went wrong with your transaction."),
                    }
                }
                return;
            } else {
                println!("Invalid input. Please try again.");
            }
        }
    }

    fn action_show_balance(self: &Self) {
        println!("Show balance from which account?");
        
        let mut user_selectable_accs: Vec<Rc<RefCell<MoneyAccount>>> = vec!();
        for ptr_acc in &self.bank_account_for_session().borrow().accounts {
            user_selectable_accs.push(ptr_acc.clone());
            println!("{}. {} ({})", user_selectable_accs.len(), ptr_acc.borrow().name, ptr_acc.borrow().id);
        }
        println!("0. Exit");

        loop {
            print!("Enter a number: ");
            let _ = std::io::stdout().flush();
            let user_input = get_user_input_as_i32();
            if user_input == 0 {
                return;
            } else if user_input <= (user_selectable_accs.len() as i32) {
                let selected_acc_index = (user_input - 1) as usize; // subtract one for zero-indexing
                let ptr_money_account = user_selectable_accs[selected_acc_index].clone();
                let mut money_account = ptr_money_account.borrow();
                println!("Your current balance is ${}", money_account.balance);
                return;
            } else {
                println!("Invalid input. Please try again.");
            }
        }
    }

    fn action_withdraw_overdraft(self: &Self) {
        
    }

    fn get_valid_actions(self: &Self, acc: Rc<RefCell<BankAccount>>) -> Vec<UserActions> {
        let mut supported_user_actions: Vec<UserActions> = vec![];
        
        let mut has_checkings_acc: bool = false;
        let mut has_savings_acc: bool = false;
        let mut has_positive_balance: bool = false;

        // Check what accounts exist
        for ptr_acc in &acc.borrow().accounts {
            let money_acc = ptr_acc.borrow();
            match money_acc.account_type  {
                MoneyAccountType::Checking => has_checkings_acc = true,
                MoneyAccountType::Savings => has_savings_acc = true,
                _ => (),
            }
            if money_acc.balance > 0 {
                has_positive_balance = true;
            }
        }
    
        if has_savings_acc || has_checkings_acc {
            supported_user_actions.push(UserActions::Deposit);
            supported_user_actions.push(UserActions::ShowBalance);
            if has_positive_balance {
                supported_user_actions.push(UserActions::Withdraw);
            }
        }
        if !has_checkings_acc {
            supported_user_actions.push(UserActions::CreateCheckings);
        }
        if !has_savings_acc {
            supported_user_actions.push(UserActions::CreateSavings);
        }
        supported_user_actions.push(UserActions::Logout);
        //supported_user_actions.push(UserActions::Exit);
        return supported_user_actions;
    }
}

fn get_user_input_as_i32() -> i32 {
    let mut user_input_buffer = String::new();
    loop {
        user_input_buffer.clear();
        let _ = std::io::stdin().read_line(&mut user_input_buffer).unwrap();
        match user_input_buffer.trim().parse::<i32>() {
            Ok(user_input) => return user_input,
            Err(err) => println!("{}! Invalid input. Please try again.", err),
        }
    }
}

fn get_user_input_as_str() -> String {
    let mut user_input_buffer = String::new();
    loop {
        user_input_buffer.clear();
        let _ = std::io::stdin().read_line(&mut user_input_buffer).unwrap();
        return String::from(user_input_buffer.trim());
    }
}