use chrono::prelude::*;
use crate::parser::parse_error::{ParseError, ArgumentParseError, DateTimeParseError};
use crate::activity::parsed_activity::ParsedActivity;

struct ActivityLine {
    line: String
}
    
impl ActivityLine {
    fn new(line: &str) -> Self {
	ActivityLine {
	    line: line.to_string()
	}
    }
    
    fn parse(&self) -> Result<ParsedActivity, ParseError> {
	let parts = self.line.split(" | ").collect::<Vec<&str>>();
	if parts.len() < 7 {
	    return Err(ParseError::TooFewArguments)
	}
  
	let start = Self::get_start(&parts);	
	let end = Self::get_end(&parts);
	
	let item_errors = Self::get_errors(vec![start.clone(), end.clone()]);
	if !item_errors.is_empty() {
	    return Err(ParseError::ArgumentErrors(item_errors))
	}
	
	Ok(ParsedActivity::from(
	    start.unwrap(),
	    end.unwrap(),
	    parts.get(3).unwrap().to_string(),
	    parts.get(4).unwrap().to_string(),
	    parts.get(5).unwrap().to_string()
	))
    }

    fn get_errors(arguments: Vec<Result<DateTime<Local>, ArgumentParseError>>) -> Vec<ArgumentParseError> {
	arguments
	    .into_iter()
	    .filter(|option| option.is_err())	 
	    .map(|option| option.unwrap_err())
	    .collect::<Vec<ArgumentParseError>>()
    }
    
    fn get_start(parts: &Vec<&str>) -> Result<DateTime<Local>, ArgumentParseError> {
	string_to_local_date(parts.get(1).unwrap())
	    .map_err(|err| ArgumentParseError::StartNotConvertableToDateTime(err.to_string()))
    }

    fn get_end(parts: &Vec<&str>) -> Result<DateTime<Local>, ArgumentParseError> {
	string_to_local_date(parts.get(2).unwrap())
	    .map_err(|err| ArgumentParseError::EndNotConvertableToDateTime(err.to_string()))
    }
}

fn string_to_local_date(string: &str) -> Result<DateTime<Local>, DateTimeParseError> {
    let naive_time = string.parse::<NaiveDateTime>()
	.map_err(|_| DateTimeParseError::NotConvertible)?;
    Ok( Local.from_local_datetime(&naive_time).unwrap() )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_throws_when_start_cannot_be_converted_to_date_time() {
	let line = ActivityLine::new(" | A2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");
	
	let parsed_line = line.parse();
	
	assert_eq!(parsed_line, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::StartNotConvertableToDateTime(DateTimeParseError::NotConvertible.to_string())
	])))
    }
    
    #[test]
    fn it_throws_when_not_enough_columns_are_given() {
	let line = ActivityLine::new(" | Bla | Bla | ");

	let parsed_line = line.parse();

	assert_eq!(parsed_line, Err(ParseError::TooFewArguments))
    }

    #[test]
    fn it_throws_when_start_and_end_cannot_be_converted_to_date_time() {
	let line = ActivityLine::new(" | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Project | Task | Description | ");
	
	let parsed_line = line.parse();
	
	assert_eq!(parsed_line, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::StartNotConvertableToDateTime(DateTimeParseError::NotConvertible.to_string()),
	    ArgumentParseError::EndNotConvertableToDateTime(DateTimeParseError::NotConvertible.to_string())
	])))
    }
    
    #[test]
    fn it_parses_a_full_timing_line() {
	let line = ActivityLine::new(" | 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");

	let parsed_line = line.parse();

	assert_eq!(parsed_line, Ok(ParsedActivity::from(
	    Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    "Project".to_string(),
	    "Task".to_string(),
	    "Description".to_string()
	)));
    }
}
