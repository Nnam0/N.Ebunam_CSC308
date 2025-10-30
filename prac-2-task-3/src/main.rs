struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn new(balance: f64) -> Self {
        BankAccount { balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${}.", amount);
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew ${}.", amount);
        } else {
            println!("Insufficient funds!");
        }
    }

    fn check_balance(&self) {
        println!("Current balance: ${}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new(100.0);

    account.check_balance();
    account.deposit(50.0);
    account.withdraw(30.0);
    account.withdraw(150.0);
    account.check_balance();
}
