use std::collections::HashMap;

use crate::journal::{Journal};

pub fn balance(journal: &Journal) -> HashMap<String, f64> {
    let sub_accounts_amounts = sub_account_balances(journal);
    let all_accounts_amounts = all_account_balances(&sub_accounts_amounts);
    return all_accounts_amounts;
}

fn sub_account_balances(journal: &Journal) -> HashMap<String, f64> {
    let mut sub_accounts_amounts = HashMap::new();

    for transaction in &journal.transactions {
        for entry in &transaction.entries {
            let sub_account_amount = sub_accounts_amounts.entry(entry.account.clone()).or_insert(0_f64);
            *sub_account_amount += entry.amount;
        }
    }
    return sub_accounts_amounts;
}

fn all_account_balances(sub_accounts_amounts: &HashMap<String, f64>) -> HashMap<String, f64>  {
    let mut all_accounts_amounts = HashMap::new();
    
    for sub_account_name in sub_accounts_amounts.keys() {
        let sub_account_amount = sub_accounts_amounts.get(sub_account_name).unwrap();

        let mut account_name_components = sub_account_name.split(":");

        let mut super_account_name = account_name_components.next().unwrap().clone().to_string();
        let super_account_amount = all_accounts_amounts.entry(super_account_name.clone()).or_insert(0_f64);
        *super_account_amount += sub_account_amount;

        for account_name_component in account_name_components {
            super_account_name.push(':');
            super_account_name.push_str(account_name_component);

            let super_account_amount = all_accounts_amounts.entry(super_account_name.clone()).or_insert(0_f64);
            *super_account_amount += sub_account_amount;
        }
    }
    return all_accounts_amounts;
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate};

    use super::*;
    use crate::journal::{Entry, Transaction};

    #[test]
    fn test_balance() {
        // Given
        let example_journal = Journal {
            transactions: vec![
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                    entries: vec![
                        Entry { account: String::from("assets:saving"), amount: -120_f64, },
                        Entry { account: String::from("assets:current"), amount: 120_f64, },
                    ]
                },
                Transaction {
                    date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), 
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
                                             (String::from("assets"), -20_f64),
                                             (String::from("assets:saving"), -120_f64),
                                             (String::from("assets:current"), 100_f64),
                                             (String::from("expenses"), 20_f64),
                                             (String::from("expenses:groceries"), 20_f64),
        ]);
        assert_eq!(expected_balance, actual_balance);
    }
}
