use chrono::{DateTime,NaiveDate, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag="type", content="n")]
pub enum Frequency {
    Daily,
    Weekly, // Once per calendar week
    EveryNDays(u16), // every N days
}