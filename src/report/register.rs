use chrono::NaiveDate;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::journal::Journal;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Posting {
    pub date: NaiveDate,
    pub account: String,
    pub amount: Decimal,
}

pub fn register(journal: &Journal, account_query: &str) -> Vec<Posting> {
    let mut register = Vec::new();

    for transaction in &journal.transactions {
        for entry in &transaction.entries {
            if entry.account.starts_with(account_query) {
                register.push(Posting {
                    date: transaction.date,
                    account: entry.account.clone(),
                    amount: entry.amount,
                });
            }
        }
    }

    register
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::journal::{Entry, Transaction};

    #[test]
    fn test_balance() {
        // Given
        let account_query = "assets";
        //
        let example_journal = Journal {
            transactions: vec![
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 9).unwrap(),
                    entries: vec![Entry {
                        account: String::from("assets:current"),
                        amount: Decimal::new(10, 0),
                    }],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 10).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:saving"),
                            amount: Decimal::new(-120, 0),
                        },
                        Entry {
                            account: String::from("assets:current"),
                            amount: Decimal::new(120, 0),
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 11).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: Decimal::new(-10, 0),
                        },
                        Entry {
                            account: String::from("expenses:groceries:vegetables"),
                            amount: Decimal::new(10, 0),
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 11).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: Decimal::new(-10, 0),
                        },
                        Entry {
                            account: String::from("expenses:groceries:fruit"),
                            amount: Decimal::new(10, 0),
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 12).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: Decimal::new(-30, 0),
                        },
                        Entry {
                            account: String::from("expenses:clothes"),
                            amount: Decimal::new(30, 0),
                        },
                    ],
                },
            ],
        };

        // When
        let actual_balance = register(&example_journal, account_query);

        // Then
        let expected_register = vec![
            Posting {
                date: NaiveDate::from_ymd_opt(2000, 1, 9).unwrap(),
                account: String::from("assets:current"),
                amount: Decimal::new(10, 0),
            },
            Posting {
                date: NaiveDate::from_ymd_opt(2000, 1, 10).unwrap(),
                account: String::from("assets:saving"),
                amount: Decimal::new(-120, 0),
            },
            Posting {
                date: NaiveDate::from_ymd_opt(2000, 1, 10).unwrap(),
                account: String::from("assets:current"),
                amount: Decimal::new(120, 0),
            },
            Posting {
                date: NaiveDate::from_ymd_opt(2000, 1, 11).unwrap(),
                account: String::from("assets:current"),
                amount: Decimal::new(-10, 0),
            },
            Posting {
                date: NaiveDate::from_ymd_opt(2000, 1, 11).unwrap(),
                account: String::from("assets:current"),
                amount: Decimal::new(-10, 0),
            },
            Posting {
                date: NaiveDate::from_ymd_opt(2000, 1, 12).unwrap(),
                account: String::from("assets:current"),
                amount: Decimal::new(-30, 0),
            },
        ];

        assert_eq!(expected_register, actual_balance);
    }
}
