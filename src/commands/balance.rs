use chrono::NaiveDate;
use clap::{value_parser, Arg, Command};

pub fn balance_command() -> Command {
    Command::new("balance")
        .arg(Arg::new("filepath").short('p').long("filepath"))
        .arg(
            Arg::new("from_date")
                .short('f')
                .long("from_date")
                .value_parser(value_parser!(NaiveDate)),
        )
        .arg(
            Arg::new("to_date")
                .short('t')
                .long("to_date")
                .value_parser(value_parser!(NaiveDate)),
        )
}

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
// pub struct Cli {
//     #[command(subcommand)]
//     pub command: Commands,
// }

// #[derive(Subcommand)]
// pub enum Commands {
//     Balance {
//         #[arg(short = 'p', long, required = true)]
//         filepath: Option<String>,
//         #[arg(short = 'f', long)]
//         from_date: Option<NaiveDate>,
//         #[arg(short = 't', long)]
//         to_date: Option<NaiveDate>,
//     },
// }
