use std::error::Error;

#[macro_use] extern crate prettytable;
use clap::{Args, Parser, Subcommand};

mod cli;
mod journal;
mod report;

fn main() -> Result<(), Box<dyn Error>> {
    let user_input = cli::Cli::parse();

    match &user_input.command {
        cli::Commands::Balance { filepath } => {
            cli::balance_handler(filepath)
        }
    }    
}
