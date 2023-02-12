use std::collections::HashMap;
use std::error::Error;
use std::fs;

use chrono::NaiveDate;
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
        #[arg(short = 'p', long, required = true)]
        filepath: Option<String>,
        #[arg(short = 'f', long)]
        from_date: Option<NaiveDate>,
        #[arg(short = 't', long)]
        to_date: Option<NaiveDate>,
    },
}

pub fn balance_handler(
    filepath_option: &Option<String>,
    from_date_option: &Option<NaiveDate>,
    to_date_option: &Option<NaiveDate>,
) {
    let input_journal: journal::Journal = load_journal(filepath_option).unwrap();

    input_journal.validate();

    let from_date = from_date_option.unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let to_date = to_date_option.unwrap_or(NaiveDate::from_ymd_opt(2200, 1, 1).unwrap());

    let balances = report::balance::balance(&input_journal, &from_date, &to_date);

    display_balances(balances);
}

fn load_journal(filepath_option: &Option<String>) -> Result<journal::Journal, Box<dyn Error>> {
    let filepath = filepath_option.as_deref().unwrap();
    let ledgerfile: String = fs::read_to_string(filepath)?.parse()?;
    let input_journal: journal::Journal = serde_json::from_str(&ledgerfile)?;
    Ok(input_journal)
}

fn display_balances(balances: HashMap<String, f64>) {
    let mut account_names: Vec<&String> = balances.keys().collect();
    account_names.sort();

    let mut table = prettytable::Table::new();
    table.set_titles(row!["Account", "Balance"]);

    for account_name in account_names {
        table.add_row(row![
            account_name.clone(),
            balances.get(account_name).unwrap().to_string()
        ]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}
