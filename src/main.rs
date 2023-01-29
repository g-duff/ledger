use std::error::Error;

#[macro_use] extern crate prettytable;
use clap::{Parser};

mod cli;
mod journal;
mod report;

fn main() -> Result<(), Box<dyn Error>> {
    let user_input = cli::Cli::parse();

    match &user_input.command {
        cli::Commands::Balance { filepath, from_date, to_date } => {
            cli::balance_handler(filepath, from_date, to_date)
        }
    }    
}
