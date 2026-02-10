#[macro_use]
extern crate prettytable;

mod cli;
mod commands;
mod model;
mod reports;

fn main() {
    cli::main();
}
