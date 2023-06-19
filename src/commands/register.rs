use std::error::Error;
use std::fs;

use clap::{Arg, ArgMatches, Command};
use prettytable::format;

use crate::journal;
use crate::report::register;

const FILEPATH: &str = "filepath";
const ACCOUNT_QUERY: &str = "account";

pub fn register_command() -> Command {
    Command::new("register")
        .arg(Arg::new(FILEPATH).short('p').long(FILEPATH))
        .arg(Arg::new(ACCOUNT_QUERY).short('a').long(ACCOUNT_QUERY))
}

pub fn register_handler(register_args: &ArgMatches) {
    let filepath = register_args.get_one::<String>(FILEPATH).expect("required");
    let account_query = register_args.get_one::<String>(ACCOUNT_QUERY).expect("required");

    let input_journal: journal::Journal = load_journal(filepath).unwrap();

    let register = register::register(&input_journal, account_query);

    display_table(&register);
}

fn load_journal(filepath: &String) -> Result<journal::Journal, Box<dyn Error>> {
    let ledgerfile: String = fs::read_to_string(filepath)?.parse()?;
    let input_journal: journal::Journal = serde_json::from_str(&ledgerfile)?;
    input_journal.validate();

    Ok(input_journal)
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
