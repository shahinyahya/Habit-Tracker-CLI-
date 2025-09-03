use chrono::{Datelike, Local, NaiveDate};


// Presenting present local time
pub fn today_local() -> NaiveDate {
    Local::now().date_naive()
}

// Representing week in iso format.
pub fn iso_week(date: NaiveDate) -> (i32, u32) {
    let w = date.iso_week();
    (w.year(), w.week())
}