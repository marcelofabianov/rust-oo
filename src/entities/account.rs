use crate::entities::transaction::Transaction;

pub struct Account {
    id: i32,
    name: String,
    initial_balance: f64,
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(id: i32, name: String, initial_balance: f64) -> Result<Account, String> {
        let account = Account {
            id,
            name: name.clone(),
            initial_balance,
            transactions: Vec::new(),
        };

        if let Err(err) = account.validate() {
            return Err(err);
        }

        Ok(account)
    }

    fn validate(&self) -> Result<(), String> {
        if self.name.len() == 0 {
            return Err(String::from("Account name cannot be empty"));
        }

        if self.initial_balance < 0.0 {
            return Err(String::from("Initial balance cannot be negative"));
        }

        Ok(())
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_initial_balance(&self) -> f64 {
        self.initial_balance
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}
