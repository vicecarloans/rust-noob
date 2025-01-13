// You cannnot have 2 owners of the same data

fn main() {
    // let mut x: i32 = 5;

    // // Transfer ownership of x to r
    // // let r: i32 = x;

    // let r: &mut i32 = &mut x;

    // *r += 1;
    // *r -= 3;

    // // You can have as many immutable references to a variable as you want
    // // println!("Value of x: {}", x);
    // // You can have only one mutable reference to a variable at a time
    // println!("Value of r: {}", r);

    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(45.5);

    // Check the balance again
    account.check_balance();

    let a: u16 = 5;
    println!("The value of a is {}", a);
    // Cannot do this because a is immutable
    // a = 10;
    
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owner by {} has a balance of {:.2}", self.owner, self.balance);
    }
}