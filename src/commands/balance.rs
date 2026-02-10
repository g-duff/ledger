use clap::{value_parser, Arg, Command};

use chrono::NaiveDate;

use super::constants::{FILEPATH, DATE_FROM, OUTPUT_FORMAT, DATE_TO, DEPTH};

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
