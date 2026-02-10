use std::error::Error;
use std::fs;

use crate::model::journal::Journal;


pub fn load_journal(filepath: &String) -> Result<Journal, Box<dyn Error>> {
    let ledgerfile: String = fs::read_to_string(filepath)?.parse()?;
    let input_journal: Journal = serde_json::from_str(&ledgerfile)?;
    input_journal.validate();

    Ok(input_journal)
}
