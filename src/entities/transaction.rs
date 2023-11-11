pub struct Transaction {
    id: i32,
    amount: f64,
    description: String,
    date: String,
    account_id: i32,
}

impl Transaction {
    pub fn new(
        id: i32,
        amount: f64,
        description: String,
        date: String,
        account_id: i32,
    ) -> Result<Transaction, String> {
        let transaction = Transaction {
            id,
            amount,
            description: description.clone(),
            date,
            account_id,
        };

        if let Err(err) = transaction.validate() {
            return Err(err);
        }

        Ok(transaction)
    }

    fn validate(&self) -> Result<(), String> {
        if self.description.len() == 0 {
            return Err(String::from("Transaction description cannot be empty"));
        }

        if self.amount == 0.0 {
            return Err(String::from("Transaction amount cannot be zero"));
        }

        Ok(())
    }

    pub fn get_account_id(&self) -> i32 {
        self.account_id
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}
