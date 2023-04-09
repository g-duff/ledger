#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod journal;
mod report;

fn main() {
    cli::main();
}
