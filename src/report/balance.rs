use std::collections::HashMap;

use crate::journal::{Entry, Journal, Transaction};

pub fn balance(journal: &Journal) -> HashMap<String, f64> {
    let mut sub_accounts_amounts = HashMap::new();

    for transaction in &journal.transactions {
        for entry in &transaction.entries {
            let sub_account_amount = sub_accounts_amounts.entry(entry.account.clone()).or_insert(0_f64);
            *sub_account_amount += entry.amount;
        }
    }
    return sub_accounts_amounts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance() {
        // Given
        let example_journal = Journal {
            transactions: vec![
                Transaction {
                    entries: vec![
                        Entry { account: String::from("assets:saving"), amount: -120_f64, },
                        Entry { account: String::from("assets:current"), amount: 120_f64, },
                    ]
                },
                Transaction {
                    entries: vec![
                        Entry { account: String::from("assets:current"), amount: -20_f64, },
                        Entry { account: String::from("expenses:groceries"), amount: 20_f64, },
                    ]
                },
            ],
        };

        // When
        let actual_balance = balance(&example_journal);

        // Then
        let expected_balance = HashMap::from([
                                             (String::from("assets:saving"), -120_f64),
                                             (String::from("assets:current"), 100_f64),
                                             (String::from("expenses:groceries"), 20_f64),
        ]);
        assert_eq!(expected_balance, actual_balance);
    }
}
