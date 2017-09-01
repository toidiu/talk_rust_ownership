#![allow(unused)]


fn main() {
    //initialize
    let alice_balance = Balance { value: 100 };
    let mut alice_acc = Account { user_name: String::from("alice"), balance: alice_balance };
    let bob_balance = Balance { value: 100 };
    let mut bob_acc = Account { user_name: "bob".to_string(), balance: bob_balance };
    check_accounts(vec![&alice_acc, &bob_acc]);

    // BAD: the ownership of `alice_balance` has already moved to `alice_acc`
    //println!("{:?}, alice_balance);


    /* BAD: by initializing transaction we extends the lifetime of `Transaction`
       and therefore the mutable reference for the entire scope of the main function.
       This restricts us from borrowing it again due to ownership rules.

       Instead we use one of the options below to restrict the scope.
    */
    //    let mut transaction = Transaction { from: &mut alice_acc, to: &mut bob_acc, amount: 20 };
    //    transfer1.execute();

    { /* TRANSACTION 1: Restrict the scope to the block. */
        let transfer = Transaction { from: &mut alice_acc, to: &mut bob_acc, amount: 10 };
        transfer.execute();
        //transfer.execute(); //once we change the API of execute to take ownership of the
        //transaction then we cant execute it more than once.
    }
    check_accounts(vec![&alice_acc, &bob_acc]);

    /* TRANSACTION 2: Restrict the scope to a single statement by creating a function */
    transfer(&mut alice_acc, &mut bob_acc, 10);
    check_accounts(vec![&alice_acc, &bob_acc]);

    /* TRANSACTION 3: Restricting scope to a single statement b/c we dont assign a variable. */
    Transaction { from: &mut alice_acc, to: &mut bob_acc, amount: 10 }.execute();
    check_accounts(vec![&alice_acc, &bob_acc]);
}


fn check_accounts(acc: Vec<&Account>) {
    println!("checking...");
    for i in acc {
        println!("{:?}", i);
    }
}

fn transfer(
    mut from: &mut Account,
    mut to: &mut Account,
    amount: u64) {
    Transaction { from, to, amount }.execute();
}

#[derive(Debug)]
struct Account {
    user_name: String,
    balance: Balance,
}

#[derive(Debug)]
struct Balance {
    value: u64,
}

#[derive(Debug)]
struct Transaction<'a> {
    from: &'a mut Account,
    to: &'a mut Account,
    amount: u64,
}

impl<'a> Transaction<'a> {
    // we want to change this to move so that we can only call execute once!!
    //fn execute(&mut self) {
    fn execute(self) {
        self.from.balance.value -= self.amount;
        self.to.balance.value += self.amount;
    }
}

