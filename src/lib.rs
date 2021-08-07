use crate::line_error::LineError;
use crate::parser::activity_line::ActivityLine;
use crate::projects::{
    project::{ProjectWithTasks, ProjectWithTasksBuilder},
    projects::{Projects, ProjectsBuilder},
    task::{Task, TaskBuilder},
    tasks::TasksBuilder,
};

mod activity;
mod line_error;
mod parser;
mod projects;

pub fn validate(text: String) -> String {
    let (_activities, errors): (Vec<_>, Vec<_>) = text
        .lines()
        .enumerate()
        .filter(|(_no, line)| line.to_string() != "".to_string())
        .map(|(no, line)| (no, ActivityLine::new(line)))
        .map(|(no, activity_line)| (no, activity_line.parse(&projects())))
        .map(|(no, activity)| activity.map_err(|e| e.at_line(no)))
        .partition(Result::is_ok);
    errors
        .into_iter()
        .map(Result::unwrap_err)
        .flatten()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn task() -> Task {
    TaskBuilder::new()
        .with_name("Some specific task".to_string())
        .build()
}

fn project() -> ProjectWithTasks {
    ProjectWithTasksBuilder::new()
        .with_name("Some specific project".to_string())
        .with_tasks(TasksBuilder::new().with_tasks(vec![task()]).build())
        .build()
}

fn projects() -> Projects {
    ProjectsBuilder::new()
        .with_projects(vec![project()])
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gives_errors_for_all_lines() {
        let lines = r#"
 | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Bla     | Bla  | Description | 

 | 2020-01-12T08:00:00  | 2020-01-12T08:30:00  | Project | Task | Description | 


 | A2020-01-12T08:00:00 | 2020-01-12T08:30:00  | Project | Bla  | Description | 
"#
        .to_string();

        let errors = validate(lines);

        let expected_line_starts = vec!["1Start", "1End", "1Project", "6Start", "6Task"];
        let line_starts = errors
            .lines()
            .map(|line| {
                line.split("|")
                    .map(|part| part.trim())
                    .take(3)
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        assert_eq!(line_starts, expected_line_starts);
    }
}
