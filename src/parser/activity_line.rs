use crate::projects::projects::Projects;
use chrono::prelude::*;
use crate::parser::parse_error::{ParseError, DateTimeParseError};
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
    
    fn parse(&self, projects: &Projects) -> Result<ParsedActivity, ParseError> {

	let parts = Self::split(&self.line)?;
  
	let start = string_to_local_date(parts.get(1).unwrap());
	let end = string_to_local_date(parts.get(2).unwrap());
	let project_and_task = projects.get_project_with_task(
	    &parts.get(3).unwrap(),
	    &parts.get(4).unwrap()
	);
	
	ParseError::from_arguments(&start, &end, &project_and_task)?;
	
	Ok(ParsedActivity::from(
	    start.unwrap(),
	    end.unwrap(),
	    parts.get(3).unwrap().to_string(),
	    parts.get(4).unwrap().to_string(),
	    parts.get(5).unwrap().to_string()
	))
    }

    fn split(line: &str) -> Result<Vec<&str>, ParseError> {
	let parts = line.split(" | ").collect::<Vec<&str>>();
	if parts.len() < 7 {
	    return Err(ParseError::TooFewArguments)
	}
	Ok(parts)
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
    use crate::parser::parse_error::ArgumentParseError;
    use crate::projects::{
	projects::ProjectsBuilder,
	project::ProjectWithTasksBuilder,
	task::TaskBuilder,
	tasks::TasksBuilder
    };
    
    fn projects() -> Projects {
  	ProjectsBuilder::new()
	    .with_projects(vec![
		ProjectWithTasksBuilder::new()
		    .with_name("Some specific project".to_string())
		    .with_tasks(TasksBuilder::new()
				.with_tasks(vec![
				    TaskBuilder::new()
					.with_name("Some specific task".to_string())
					.build()
				])
				.build()
		    )
 		    .build()
		    
	    ])
	    .build()
    }

    #[test]
    fn it_throws_when_start_cannot_be_converted_to_date_time() {
	let line = ActivityLine::new(" | A2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");
	
	let parsed_line = line.parse(&projects());
	
	assert_eq!(parsed_line, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::Start(DateTimeParseError::NotConvertible)
	])))
    }
    
    #[test]
    fn it_throws_when_not_enough_columns_are_given() {
	let line = ActivityLine::new(" | Bla | Bla | ");

	let parsed_line = line.parse(&projects());

	assert_eq!(parsed_line, Err(ParseError::TooFewArguments))
    }

    #[test]
    fn it_throws_when_start_and_end_cannot_be_converted_to_date_time() {
	let line = ActivityLine::new(" | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Project | Task | Description | ");
	
	let parsed_line = line.parse(&projects());
	
	assert_eq!(parsed_line, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::Start(DateTimeParseError::NotConvertible),
	    ArgumentParseError::End(DateTimeParseError::NotConvertible)
	])))
    }
    
    #[test]
    fn it_parses_a_full_timing_line() {
	let line = ActivityLine::new(" | 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");

	let parsed_line = line.parse(&projects());

	assert_eq!(parsed_line, Ok(ParsedActivity::from(
	    Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    "Project".to_string(),
	    "Task".to_string(),
	    "Description".to_string()
	)));
    }
}
