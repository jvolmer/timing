use crate::activity::activity::Activity;
use crate::projects::projects::Projects;
use chrono::prelude::*;
use crate::parser::parse_error::{ParseError, DateTimeParseError};

struct ActivityLine {
    line: String
}
    
impl ActivityLine {
    fn new(line: &str) -> Self {
	ActivityLine {
	    line: line.to_string()
	}
    }
    
    fn parse(&self, projects: &Projects) -> Result<Activity, ParseError> {
	let parts = Self::split(&self.line)?;
  
	let start = string_to_local_date(parts.get(1).unwrap());
	let end = string_to_local_date(parts.get(2).unwrap());
	let project_and_task = projects.get_project_with_task(
	    &parts.get(3).unwrap(),
	    &parts.get(4).unwrap()
	);
	let description = parts.get(5).unwrap().to_string();
	
	ParseError::from_arguments(&start, &end, &project_and_task)?;

	let (project, task) = project_and_task.unwrap();
	Ok(Activity::from(
	    start.unwrap(),
	    end.unwrap(),
	    project,
	    task,
	    description
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
	project::{Project, ProjectWithTasks, ProjectWithTasksBuilder},
	task::{Task, TaskBuilder},
	tasks::TasksBuilder
    };

    fn task() -> Task {
	TaskBuilder::new()
	    .with_name("Some specific task".to_string())
	    .build()
    }
    
    fn project() -> ProjectWithTasks {
	ProjectWithTasksBuilder::new()
	    .with_name("Some specific project".to_string())
	    .with_tasks(TasksBuilder::new()
			.with_tasks(vec![
			    task()
			])
			.build()
	    )
 	    .build()
    }
    
    fn projects() -> Projects {
  	ProjectsBuilder::new()
	    .with_projects(vec![
		project()
	    ])
	    .build()
    }

    #[test]
    fn it_throws_when_start_cannot_be_converted_to_date_time() {
	let line = ActivityLine::new(" | A2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");
	
	let activity = line.parse(&projects());
	
	assert_eq!(activity, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::Start(DateTimeParseError::NotConvertible)
	])))
    }
    
    #[test]
    fn it_throws_when_not_enough_columns_are_given() {
	let line = ActivityLine::new(" | Bla | Bla | ");

	let activity = line.parse(&projects());

	assert_eq!(activity, Err(ParseError::TooFewArguments))
    }

    #[test]
    fn it_throws_when_start_and_end_cannot_be_converted_to_date_time() {
	let line = ActivityLine::new(" | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Project | Task | Description | ");
	
	let activity = line.parse(&projects());
	
	assert_eq!(activity, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::Start(DateTimeParseError::NotConvertible),
	    ArgumentParseError::End(DateTimeParseError::NotConvertible)
	])))
    }
    
    #[test]
    fn it_parses_a_full_timing_line() {
	let line = ActivityLine::new(" | 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ");

	let parsed_line = line.parse(&projects());

	assert_eq!(parsed_line, Ok(Activity::from(
	    Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    Project::new(&project()),
	    task(),
	    "Description".to_string()
	)));
    }

    #[test]
    fn it_finds_suitable_project_and_task_in_projects() {
	let line = ActivityLine::new(" | 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | specific project | specific task | some description | ");
	let task_to_be_found = TaskBuilder::new()
	    .with_name("Some specific task".to_string())
	    .build();
	let project_to_be_found = ProjectWithTasksBuilder::new()
	    .with_name("Some specific project".to_string())
	    .with_tasks(TasksBuilder::new()
			.with_tasks(vec![
			    TaskBuilder::new()
				.with_name("Some task".to_string())
				.build(),
			    task_to_be_found.clone()])
			.build())
	    .build();
	let projects = ProjectsBuilder::new()
	    .with_projects(vec![
		ProjectWithTasksBuilder::new()
		    .with_name("Urgent".to_string())
		    .build(),
		project_to_be_found.clone()
	    ])
	    .build();

	let activity = line.parse(&projects);

	assert_eq!(activity, Ok(Activity::from(
	    Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    Project::new(&project_to_be_found),
	    task_to_be_found,
	    "some description".to_string())));
    }
}
