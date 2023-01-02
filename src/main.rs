mod journal;
mod report;

use std::fs;
use std::error::Error;

use serde_json;

const JOURNAL_FILEPATH: &str = "ledger.json";

fn main() -> Result<(), Box<dyn Error>> {
    let ledgerfile: String = fs::read_to_string(JOURNAL_FILEPATH)?.parse()?;
    let input_journal: journal::Journal = serde_json::from_str(&ledgerfile)?;
    println!("{:?}", input_journal);
    Ok(())
}
