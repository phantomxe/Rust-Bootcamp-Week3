

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&mut self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
} 

impl Account for BankAccount {
    fn balance(&mut self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }
}


fn main() {
    let mut acc1 = BankAccount {
        account_number: 1701,
        holder_name: "User1".to_string(),
        balance: 0.0
    };

    let mut acc2 = BankAccount {
        account_number: 1702,
        holder_name: "User2".to_string(),
        balance: 300.0
    };

    acc1.deposit(5000.0);
    acc2.withdraw(55.0);

    println!("Acc1 Balance: {:?}", acc1.balance());
    println!("Acc2 Balance: {:?}", acc2.balance()); 
}
