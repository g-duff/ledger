#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod journal;
mod reports;

fn main() {
    cli::main();
}
