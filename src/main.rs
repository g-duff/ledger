#[macro_use]
extern crate prettytable;
use clap::Command;

mod commands;
mod journal;
mod report;

fn main() {
    let user_input = Command::new("ledger")
        .subcommand(commands::balance::balance_command())
        .get_matches();

    match user_input.subcommand() {
        Some(("balance", report_args)) => commands::balance::balance_handler(report_args),
        _ => println!("no command given"),
    }
}
