use thiserror::Error;
use chrono::{DateTime,NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag="type", content="n")]
pub enum Frequency {
    Daily,
    Weekly, // Once per calendar week
    EveryNDays(u16), // every N days
}

#[derive(Debug, Error)]
pub enum ParseFreqError {
    #[error("Invalid Frequency string..")]
    Invalid,
}

impl FromStr for Frequency {
    type Err = ParseFreqError;

    // Create function for identifying string for frequency in lower case.
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // trim and convert string to small case
        let s = s.trim().to_lowercase();
        if s == "daily" { return Ok(Frequency::Daily); }
        if s == "weekly" { return Ok(Frequency::Weekly); }
        if let Some(rest) = s.strip_prefix("every:") {
            let n: u16 = rest.parse().map_err(|_| ParseFreqError::Invalid)?;
            if n == 0 { return Err(ParseFreqError::Invalid); }
            return Ok(Frequency::EveryNDays(n));
        }
        Err(ParseFreqError::Invalid)       
    }
}

/*
* Data Structures for Habit, Completion, and data.
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub frequency: Frequency,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc> ,
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Completion {
    pub habit_id: Uuid,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    pub schema_version: u32,
    pub habits: Vec<Habit>,
    pub completions: Vec<Completion>,
}

impl Default for DataFile {
    fn default() -> Self {
        Self { schema_version: 1, habits: vec![], completions: vec![] }
    }
}