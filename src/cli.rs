use clap::command;

use crate::commands;

pub fn main() {
    let user_input = command!()
        .subcommand(commands::balance::balance_command())
        .subcommand(commands::register::register_command())
        .get_matches();

    match user_input.subcommand() {
        Some(("balance", report_args)) => commands::balance::balance_handler(report_args),
        Some(("register", register_args)) => commands::register::register_handler(register_args),
        _ => println!("no command given"),
    }
}
