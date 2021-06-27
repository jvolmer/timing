use chrono::prelude::*;
use crate::parser::parse_error::ParseError;
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
	    return Err(ParseError::new("Too few arguments given"))
	}
	Ok(ParsedActivity::from(
	    string_to_local_date(parts.get(1).unwrap()).unwrap(),
	    string_to_local_date(parts.get(2).unwrap()).unwrap(),
	    parts.get(3).unwrap().to_string(),
	    parts.get(4).unwrap().to_string(),
	    parts.get(5).unwrap().to_string()
	))
    }	
}

fn string_to_local_date(string: &str) -> Result<DateTime<Local>, ParseError> {
    let naive_time = string.parse::<NaiveDateTime>()?;
    Ok( Local.from_local_datetime(&naive_time).unwrap() )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_throws_when_not_enough_columns_are_given() {
	let line = ActivityLine::new(" | Bla | Bla | ");

	let parsed_line = line.parse();

	assert_eq!(parsed_line, Err(ParseError::new("Too few arguments given")))
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
