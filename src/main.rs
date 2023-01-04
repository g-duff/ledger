#[macro_use] extern crate prettytable;
use clap::{Args, Parser, Subcommand};

mod cli;
mod journal;
mod report;

fn main() {
    let user_input = cli::Cli::parse();

    match &user_input.command {
        cli::Commands::Balance(args) => {
            if let Some(filename) = args.filename.as_deref() {
                println!("{}", filename.to_string());
                cli::balance_handler(filename.to_string());
            }
        }
    }    
}
