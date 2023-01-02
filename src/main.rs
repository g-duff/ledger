mod journal;
mod report;

use std::fs;
use std::error::Error;

#[macro_use] extern crate prettytable;
use prettytable::format;
use serde_json;

const JOURNAL_FILEPATH: &str = "ledger.json";

fn main() -> Result<(), Box<dyn Error>> {
    let ledgerfile: String = fs::read_to_string(JOURNAL_FILEPATH)?.parse()?;
    let input_journal: journal::Journal = serde_json::from_str(&ledgerfile)?;

    let balances = report::balance::balance(&input_journal);

    let mut account_names: Vec<&String> = balances.keys().collect();
    account_names.sort();

    let mut table = prettytable::Table::new();
    table.set_titles(row!["Account", "Balance"]);

    for account_name in account_names {
        table.add_row(row![account_name.clone(), balances.get(account_name).unwrap().to_string()]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
    Ok(())
}