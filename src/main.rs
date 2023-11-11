mod entities {
    pub mod account;
    pub mod transaction;
}

use entities::account::Account;
use entities::transaction::Transaction;

fn main() {
    let account = Account::new(1, String::from("Checking"), 100.0);
    let transaction = Transaction::new(
        1,
        100.0,
        String::from("Initial deposit"),
        String::from("2019-01-01"),
        1,
    );

    account.unwrap().add_transaction(transaction.unwrap());
}
