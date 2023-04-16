use std::collections::HashMap;

use chrono::NaiveDate;

use crate::journal::{Entry, Journal};

pub fn balance(
    journal: &Journal,
    depth: &usize,
    from_date: &NaiveDate,
    to_date: &NaiveDate,
) -> HashMap<String, f64> {
    let entries = journal.entries_between_dates(from_date, to_date);
    account_balances(&entries, depth)
}

fn account_balances(entries: &Vec<&Entry>, depth: &usize) -> HashMap<String, f64> {
    let mut accounts_amounts = HashMap::new();
    let mut account_name = String::new();

    for entry in entries {
        for account_name_component in entry.account.split(':').take(*depth) {
            account_name.push_str(account_name_component);

            let super_account_amount = accounts_amounts
                .entry(account_name.clone())
                .or_insert(0_f64);
            *super_account_amount += entry.amount;

            account_name.push(':');
        }

        account_name = String::default();
    }

    accounts_amounts
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::journal::{Entry, Transaction};

    #[test]
    fn test_balance() {
        // Given
        let from_date = NaiveDate::from_ymd_opt(2000, 1, 10).unwrap();
        let to_date = NaiveDate::from_ymd_opt(2000, 1, 11).unwrap();
        let example_journal = Journal {
            transactions: vec![
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 9).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: 10_f64,
                        },
                        Entry {
                            account: String::from("expenses:travel"),
                            amount: 10_f64,
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 10).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:saving"),
                            amount: -120_f64,
                        },
                        Entry {
                            account: String::from("assets:current"),
                            amount: 120_f64,
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 11).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: -20_f64,
                        },
                        Entry {
                            account: String::from("expenses:groceries"),
                            amount: 20_f64,
                        },
                    ],
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 12).unwrap(),
                    entries: vec![
                        Entry {
                            account: String::from("assets:current"),
                            amount: -30_f64,
                        },
                        Entry {
                            account: String::from("expenses:clothes"),
                            amount: 30_f64,
                        },
                    ],
                },
            ],
        };

        // When
        let actual_balance = balance(&example_journal, &from_date, &to_date);

        // Then
        let expected_balance = HashMap::from([
            (String::from("assets"), -20_f64),
            (String::from("assets:saving"), -120_f64),
            (String::from("assets:current"), 100_f64),
            (String::from("expenses"), 20_f64),
            (String::from("expenses:groceries"), 20_f64),
        ]);
        assert_eq!(expected_balance, actual_balance);
    }
}
