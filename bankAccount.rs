#[derive(Debug)]
struct BankAccount {
    balance: f64,
    accountNumber: String,
    accountHolder: String
}

impl BankAccount {
    fn new(accountHolder: &str, accountNumber: &str, initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
            accountHolder: accountHolder.to_string(),
            accountNumber: accountNumber.to_string()
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if self.balance  >= amount {
            self.balance -= amount;
            Ok(amount)
        }
        else {
            Err("insufficient balance".to_string())
        }
    }

    fn getBalance(&self) -> f64 {
        return self.balance;
    }
}

fn main() {
    let mut account = BankAccount::new("Auroshis", "1289749923", 10000000.0);
    account.deposit(1008980.0);
    match account.withdraw(13223.2222) {
        Ok(amount) => println!("Successfully withdrawn {}", amount),
        Err(error) => println!("{}", error),
    }
    println!("Current balance {}", account.getBalance());
}