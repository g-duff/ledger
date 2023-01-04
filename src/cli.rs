use std::fs;
use std::error::Error;

use clap::{Args, Parser, Subcommand};
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
    Balance(BalanceOptions),
}

#[derive(Args)]
pub struct BalanceOptions {
    pub filename: Option<String>,
}

pub fn balance_handler(journal_filepath: String) -> Result<(), Box<dyn Error>> {
    let ledgerfile: String = fs::read_to_string(journal_filepath)?.parse()?;
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
