struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ₦{:.2}. New balance: ₦{:.2}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew ₦{:.2}. New balance: ₦{:.2}", amount, self.balance);
        } else {
            println!("Insufficient funds to withdraw ₦{:.2}. Current balance: ₦{:.2}", amount, self.balance);
        }
    }

    fn check_balance(&self) {
        println!("{}'s current balance: ₦{:.2}", self.owner, self.balance);
    }
}

use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter account owner's name: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let owner = input.trim().to_string();

    print!("Enter initial balance: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let balance: f64 = input.trim().parse().expect("Please enter a valid number for balance");

    let mut account = BankAccount { owner, balance };
    account.check_balance();

    print!("Enter amount to deposit: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let deposit_amt: f64 = input.trim().parse().expect("Please enter a valid number for deposit");
    account.deposit(deposit_amt);

    print!("Enter amount to withdraw: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let withdraw_amt: f64 = input.trim().parse().expect("Please enter a valid number for withdrawal");
    account.withdraw(withdraw_amt);

    account.check_balance();
}
