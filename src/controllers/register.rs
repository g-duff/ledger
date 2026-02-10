use clap::ArgMatches;
use prettytable::format;
use serde_json;

use crate::model::journal::Journal;
use crate::commands::constants::{FILEPATH, OUTPUT_FORMAT, ACCOUNT_QUERY};
use crate::reports::register;

use super::utils;


pub fn register_handler(register_args: &ArgMatches) {
    let filepath = register_args.get_one::<String>(FILEPATH).expect("required");
    let account_query = register_args
        .get_one::<String>(ACCOUNT_QUERY)
        .expect("required");

    let input_journal: Journal = utils::load_journal(filepath).unwrap();

    let register = register::register(&input_journal, account_query);

    match register_args
        .get_one::<String>(OUTPUT_FORMAT)
        .expect("Default is table")
        .as_str()
    {
        "json" => display_json(&register),
        "table" => display_table(&register),
        _ => unreachable!(),
    }
}

fn display_json(register: &Vec<register::Posting>) {
    println!("{}", serde_json::to_string_pretty(&register).unwrap())
}

fn display_table(register: &Vec<register::Posting>) {
    let mut table = prettytable::Table::new();
    table.set_titles(row!["Date", "Amount"]);

    for posting in register {
        table.add_row(row![
            posting.date,
            r -> format!("{:.2}", posting.amount)
        ]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}
