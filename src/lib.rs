use crate::projects::{
    projects::{Projects, ProjectsBuilder},
    project::{ProjectWithTasks, ProjectWithTasksBuilder},
    task::{Task, TaskBuilder},
    tasks::TasksBuilder
};
use crate::parser::activity_line::ActivityLine;

mod projects;
mod parser;
mod activity;

pub fn validate(lines: std::slice::Iter<String>) -> String {
    let (activities, errors): (Vec<_>, Vec<_>) = lines
	.map(|line| ActivityLine::new(line))
	.map(|activity_line| activity_line.parse(&projects()))
	.partition(Result::is_ok);
    let res = errors.into_iter()
	.map(Result::unwrap_err)
	.map(|error| error.to_string())
	.collect();
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
	    " | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Bla | Bla | Description | ".to_string()
	];

	let errors = validate(lines.iter());

	println!("{}", errors);
	assert_eq!(errors.matches("\n").count(), 2);
	assert_eq!(errors.contains("Start"), true);
	assert_eq!(errors.contains("End"), true);
	assert_eq!(errors.contains("Project"), true);
    }
}
