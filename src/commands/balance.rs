use std::collections::HashMap;
use std::error::Error;
use std::fs;

use chrono::NaiveDate;
use clap:: { ArgMatches, Arg, Command, value_parser };
use prettytable::format;
use serde_json;

use crate::journal;
use crate::report;

pub fn balance_command() -> Command {
    Command::new("balance")
        .arg(Arg::new("filepath").short('p').long("filepath"))
        .arg(
            Arg::new("from_date")
                .short('f')
                .long("from_date")
                .value_parser(value_parser!(NaiveDate)),
        )
        .arg(
            Arg::new("to_date")
                .short('t')
                .long("to_date")
                .value_parser(value_parser!(NaiveDate)),
        )
}

pub fn balance_handler(report_args: &ArgMatches) {
    let filepath_option = report_args.get_one::<String>("filepath").expect("required");
    let input_journal: journal::Journal = load_journal(filepath_option).unwrap();

    input_journal.validate();

    let from_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let to_date = NaiveDate::from_ymd_opt(2200, 1, 1).unwrap();

    let balances = report::balance::balance(&input_journal, &from_date, &to_date);

    display_balances(balances);
}

fn load_journal(filepath: &String) -> Result<journal::Journal, Box<dyn Error>> {
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
