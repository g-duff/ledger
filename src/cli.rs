use clap::command;

use crate::commands;

pub fn main() {
    let user_input = command!()
        .subcommand(commands::balance::balance_command())
        .get_matches();

    match user_input.subcommand() {
        Some(("balance", report_args)) => commands::balance::balance_handler(report_args),
        _ => println!("no command given"),
    }
}
