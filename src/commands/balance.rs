use std::collections::HashMap;

use chrono::NaiveDate;
use clap::{value_parser, Arg, ArgMatches, Command};
use prettytable::format;
use rust_decimal::prelude::Decimal;
use serde::Serialize;
use serde_json;

use crate::model::{journal};
use crate::reports;

use super::common::{FILEPATH, DATE_FROM, OUTPUT_FORMAT, DATE_TO};

pub const DEPTH: &str = "depth";

pub fn balance_command() -> Command {
    Command::new("balance")
        .arg(Arg::new(FILEPATH).short('p').long(FILEPATH))
        .arg(
            Arg::new(DEPTH)
                .short('d')
                .long(DEPTH)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new(DATE_FROM)
                .short('f')
                .long(DATE_FROM)
                .value_parser(value_parser!(NaiveDate)),
        )
        .arg(
            Arg::new(DATE_TO)
                .short('t')
                .long(DATE_TO)
                .value_parser(value_parser!(NaiveDate)),
        )
        .arg(
            Arg::new(OUTPUT_FORMAT)
                .short('o')
                .long(OUTPUT_FORMAT)
                .value_parser(["table", "json"])
                .default_value("table"),
        )
}

pub fn balance_handler(report_args: &ArgMatches) {
    let filepath = report_args.get_one::<String>(FILEPATH).expect("required");
    let input_journal: journal::Journal = journal::load_journal(filepath).unwrap();

    let depth = report_args.get_one::<usize>(DEPTH).unwrap_or(&usize::MAX);

    let from_date = report_args
        .get_one::<NaiveDate>(DATE_FROM)
        .unwrap_or(&NaiveDate::MIN);
    let to_date = report_args
        .get_one::<NaiveDate>(DATE_TO)
        .unwrap_or(&NaiveDate::MAX);

    let balances = reports::balance::balance(&input_journal, depth, from_date, to_date);

    match report_args
        .get_one::<String>(OUTPUT_FORMAT)
        .expect("Default is table")
        .as_str()
    {
        "json" => display_json(&balances),
        "table" => display_table(&balances),
        _ => unreachable!(),
    }
}

fn display_json(balances: &HashMap<String, Decimal>) {
    #[derive(Serialize)]
    struct Account {
        account: String,
        balance: Decimal,
    }

    let mut account_names: Vec<&String> = balances.keys().collect();
    account_names.sort();
    let accounts: Vec<Account> = account_names
        .iter()
        .map(|account_name| {
            let name = account_name.to_string();
            Account {
                balance: *balances.get(&name).unwrap(),
                account: name,
            }
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&accounts).unwrap())
}

fn display_table(balances: &HashMap<String, Decimal>) {
    let mut account_names: Vec<&String> = balances.keys().collect();
    account_names.sort();

    let mut table = prettytable::Table::new();
    table.set_titles(row!["Account", "Balance"]);

    for account_name in account_names {
        table.add_row(row![
            account_name.clone(),
            r -> format!("{:.2}", balances.get(account_name).unwrap())
        ]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}
