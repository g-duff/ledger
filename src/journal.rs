use std::error::Error;
use std::fs;

use chrono::NaiveDate;
use rust_decimal::prelude::Decimal;
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
    pub amount: Decimal,
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
        entries
    }

    pub fn validate(&self) {

        let mut balance: Decimal;
        for transaction in &self.transactions {
            balance = Decimal::ZERO;
            for entry in &transaction.entries {
                balance += entry.amount;
            }
            if !balance.is_zero() {
                panic!("Transaction on {:?} is unbalanced", transaction.date);
            }
        }
    }
}

pub fn load_journal(filepath: &String) -> Result<Journal, Box<dyn Error>> {
    let ledgerfile: String = fs::read_to_string(filepath)?.parse()?;
    let input_journal: Journal = serde_json::from_str(&ledgerfile)?;
    input_journal.validate();

    Ok(input_journal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Transaction on 2000-01-09 is unbalanced")]
    fn test_balance() {
        // Given
        let example_journal = Journal {
            transactions: vec![
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 9).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: Decimal::new(10, 0),
                        }
                    ],
                },
            ],
        };

        // When/Then
        example_journal.validate();
    }
}
