use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Journal {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub date: NaiveDate,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub account: String,
    pub amount: f64,
}

impl Journal {
    pub fn entries_between_dates(&self, from_date: &NaiveDate, to_date: &NaiveDate) -> Vec<&Entry> {
        let mut entries = Vec::new();

        for transaction in &self.transactions {
            if transaction.date >= *from_date && transaction.date <= *to_date {
                for entry in &transaction.entries {
                    entries.push(entry);
                }
            }
        }
        return entries;
    }

    pub fn validate(&self) {

        let mut balance: f64;
        for transaction in &self.transactions {
            balance = 0_f64;
            for entry in &transaction.entries {
                balance += entry.amount;
            }
            if (balance * 100_f64).round() != 0_f64 {
                panic!("Transaction on {:?} is unbalanced", transaction.date);
            }
        }
    }

}
