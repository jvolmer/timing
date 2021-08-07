use crate::projects::{
    projects::{Projects, ProjectsBuilder},
    project::{ProjectWithTasks, ProjectWithTasksBuilder},
    task::{Task, TaskBuilder},
    tasks::TasksBuilder
};
use crate::parser::activity_line::ActivityLine;
use crate::line_error::LineError;

mod projects;
mod parser;
mod activity;
mod line_error;

pub fn validate(lines: std::slice::Iter<String>) -> String {
    let (activities, errors): (Vec<_>, Vec<_>) = lines
	.map(|line| ActivityLine::new(line))
	.map(|activity_line| activity_line.parse(&projects()))
	.enumerate()
	.map(|(i, activity)| activity.map_err(|e| e.at_line(i)))
	.partition(Result::is_ok);
    let res = errors.into_iter()
	.map(Result::unwrap_err)
	.flatten()
	.map(|error| error.to_string())
	.collect::<Vec<_>>()
	.join("\n");
    res	
}

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gives_errors_for_all_lines() {
	let lines = vec![
	    " | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Bla | Bla | Description | ".to_string(),
	    " | 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Task | Description | ".to_string(),
	    " | A2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Project | Bla | Description | ".to_string()

	];

	let errors = validate(lines.iter());

	println!("{}", errors);
	let expected_line_starts = vec!["0  Start ", "0  End ", "0  Project ", "2  Start ", "2  Task "];
	let line_starts = errors
	    .split("\n")
	    .map(|line| line
		 .split("|")
		 .take(2)
		 .collect::<String>())
	    .collect::<Vec<_>>();
	assert_eq!(line_starts, expected_line_starts);
    }
}
