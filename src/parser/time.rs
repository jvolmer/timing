use crate::activity::time::{End, Start};
use crate::parser::parse_error::DateTimeParseError;
use chrono::prelude::*;

pub fn start(string: &str) -> Result<Start, DateTimeParseError> {
    Ok(Start::new(string_to_local_date(string)?))
}

pub fn end(string: &str) -> Result<End, DateTimeParseError> {
    Ok(End::new(string_to_local_date(string)?))
}

fn string_to_local_date(string: &str) -> Result<DateTime<Local>, DateTimeParseError> {
    let naive_time = string
        .parse::<NaiveDateTime>()
        .map_err(|_| DateTimeParseError::NotConvertible)?;
    Ok(Local.from_local_datetime(&naive_time).unwrap())
}
