use clap::command;

use crate::commands;
use crate::controllers;

pub fn main() {
    let user_input = command!()
        .subcommand(commands::balance::balance_command())
        .subcommand(commands::register::register_command())
        .get_matches();

    match user_input.subcommand() {
        Some(("balance", balance_args)) => controllers::balance::balance_handler(balance_args),
        Some(("register", register_args)) => controllers::register::register_handler(register_args),
        _ => println!("no command given"),
    }
}
