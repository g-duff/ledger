use std::fs;
use std::error::Error;

use chrono::{NaiveDate};
use clap::{Parser, Subcommand};
use prettytable::format;
use serde_json;

use crate::journal;
use crate::report;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Balance {
        #[arg(short, long, required = true)]
        filepath: Option<String>,
    },
}

pub fn balance_handler(filepath_option: &Option<String>) -> Result<(), Box<dyn Error>> {

    let filepath = filepath_option.as_deref().expect("balance filepath should be required by clap");
    let ledgerfile: String = fs::read_to_string(filepath)?.parse()?;
    let input_journal: journal::Journal = serde_json::from_str(&ledgerfile)?;

    let from_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();

    let balances = report::balance::balance(&input_journal, &from_date);

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
