#[macro_use]
extern crate prettytable;

mod cli;
mod model;
mod commands;
mod controllers;
mod reports;

fn main() {
    cli::main();
}
