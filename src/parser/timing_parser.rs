use chrono::prelude::*;
use crate::parser::parse_error::ParseError;

#[derive(Debug, PartialEq)]
struct ParsedTiming {
    start: DateTime<Local>,
    end: DateTime<Local>,
    project: String,
    task: String,
    description: String
}

struct TimingLine {
    line: String
}
    
impl TimingLine {
    fn new(line: &str) -> Self {
	TimingLine {
	    line: line.to_string()
	}
    }
    
    fn parse(&self) -> Result<ParsedTiming, ParseError> {
	let parts = self.line.split(" | ").collect::<Vec<&str>>();
	if parts.len() < 7 {
	    return Err(ParseError::new("Too few arguments given".to_string()))
	}
	Ok(ParsedTiming {
	    start: string_to_local_date(parts.get(1).unwrap()).unwrap(),
	    end: string_to_local_date(parts.get(2).unwrap()).unwrap(),
	    project: parts.get(3).unwrap().to_string(),
	    task :parts.get(4).unwrap().to_string(),
	    description: parts.get(5).unwrap().to_string()
	})
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
	let line = TimingLine::new(" | Bla | Bla | ");

	let parsed_line = line.parse();

	assert_eq!(parsed_line, Err(ParseError::new("Too few arguments given".to_string())))
    }

    #[test]
    fn it_parses_a_full_timing_line() {
	let line = TimingLine::new(" | 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");

	let parsed_line = line.parse();

	assert_eq!(parsed_line, Ok(ParsedTiming {
	    start: Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    end: Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    project: "Project".to_string(),
	    task: "Task".to_string(),
	    description: "Description".to_string()
	}));
    }
}
