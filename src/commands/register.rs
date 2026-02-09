use clap::{Arg, ArgMatches, Command};
use prettytable::format;

use crate::journal;
use crate::reports::register;

const ACCOUNT_QUERY: &str = "account";
const FILEPATH: &str = "filepath";
const OUTPUT_FORMAT: &str = "output-format";

pub fn register_command() -> Command {
    Command::new("register")
        .arg(Arg::new(FILEPATH).short('p').long(FILEPATH))
        .arg(Arg::new(ACCOUNT_QUERY).short('a').long(ACCOUNT_QUERY))
        .arg(
            Arg::new(OUTPUT_FORMAT)
                .short('o')
                .long(OUTPUT_FORMAT)
                .value_parser(["table", "json"])
                .default_value("table"),
        )
}

pub fn register_handler(register_args: &ArgMatches) {
    let filepath = register_args.get_one::<String>(FILEPATH).expect("required");
    let account_query = register_args
        .get_one::<String>(ACCOUNT_QUERY)
        .expect("required");

    let input_journal: journal::Journal = journal::load_journal(filepath).unwrap();

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
