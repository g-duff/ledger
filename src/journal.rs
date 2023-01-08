use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Journal {
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub date: NaiveDate,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub account: String,
    pub amount: f64,
}

