use crate::projects::project_error::ProjectError;
use crate::projects::task::Task;
use crate::projects::project::Project;
use crate::projects::projects::Projects;
use chrono::prelude::*;
use crate::parser::parse_error::{ParseError, ArgumentParseError, DateTimeParseError};
use crate::activity::parsed_activity::ParsedActivity;

#[derive(Clone, Debug)]
enum Argument {
    DateTime(DateTime<Local>),
    ProjectAndTask((Project, Task)),
}

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
	let parts = self.line.split(" | ").collect::<Vec<&str>>();
	if parts.len() < 7 {
	    return Err(ParseError::TooFewArguments)
	}
  
	let start = Self::get_start(&parts);	
	let end = Self::get_end(&parts);
	let project_and_task = Self::get_project_and_task(&parts, &projects);
	
	let item_errors = Self::get_errors(
	    start.clone(),
	    end.clone(),
	    project_and_task.clone()
	);
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

    fn get_errors(start: Result<DateTime<Local>, DateTimeParseError>,
		  end: Result<DateTime<Local>, DateTimeParseError>,
		  project_and_task: Result<(Project, Task), ProjectError>)
		  -> Vec<ArgumentParseError> {
	let vec = vec![
	    start
		.map(|start| Argument::DateTime(start))
		.map_err(|err| ArgumentParseError::StartNotConvertableToDateTime(err.to_string())),
	    end
		.map(|end| Argument::DateTime(end))
		.map_err(|err| ArgumentParseError::EndNotConvertableToDateTime(err.to_string())),
	    project_and_task
		.map(|pt| Argument::ProjectAndTask(pt))
		.map_err(|_| ArgumentParseError::ProjectAndTask)
	];
	vec.into_iter()
	    .filter(|option| option.is_err())	 
	    .map(|option| option.unwrap_err())
	    .collect::<Vec<ArgumentParseError>>()
    }
    
    fn get_start(parts: &Vec<&str>) -> Result<DateTime<Local>, DateTimeParseError> {
	string_to_local_date(parts.get(1).unwrap())
    }

    fn get_end(parts: &Vec<&str>) -> Result<DateTime<Local>, DateTimeParseError> {
	string_to_local_date(parts.get(2).unwrap())
    }

    fn get_project_and_task(parts: &Vec<&str>, projects: &Projects) -> Result<(Project, Task), ProjectError> {
	let project_string = parts.get(3).unwrap().to_string();
	let task_string = parts.get(4).unwrap().to_string();
	projects.get_project_with_task(&project_string, &task_string)
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
	    ArgumentParseError::StartNotConvertableToDateTime(DateTimeParseError::NotConvertible.to_string())
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
	    ArgumentParseError::StartNotConvertableToDateTime(DateTimeParseError::NotConvertible.to_string()),
	    ArgumentParseError::EndNotConvertableToDateTime(DateTimeParseError::NotConvertible.to_string())
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
