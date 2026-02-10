use clap::{Arg, Command};

use super::constants::{FILEPATH, OUTPUT_FORMAT, ACCOUNT_QUERY};

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
