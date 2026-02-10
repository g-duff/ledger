use std::error::Error;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use crate::model::journal::Journal;

pub fn load_journal(filepath: &String)  -> Result<Journal, Box<dyn Error>> {

    let ext: Option<&str> = Path::new(filepath).extension().and_then(OsStr::to_str);
    let ledgerfile: String = fs::read_to_string(filepath)?.parse()?;
    let input_journal: Journal = match ext {
        Some("json") => serde_json::from_str(&ledgerfile)?,
        Some("yaml") => serde_yaml::from_str(&ledgerfile)?,
        Some("yml") => serde_yaml::from_str(&ledgerfile)?,
        Some(&_) => panic!("Only file extensions json, yaml and yaml are supported"),
        None => panic!("No file extension"),
    };
    input_journal.validate();

    Ok(input_journal)
}
