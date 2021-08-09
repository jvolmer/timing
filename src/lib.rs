use crate::line_error::LineError;
use crate::parser::activity_line::ActivityLine;
use crate::projects_and_tasks::{harvest::projects::HarvestProjectAssignments, projects::Projects};
use crate::validation::activity::Activity;

mod line_error;
mod parser;
pub mod projects_and_tasks;
mod validation;

pub fn validate(text: &str, projects: &Projects) -> String {
    let (_activities, errors) = parse(text, projects);
    combine_errors(errors)
}

type ActivityParseResult = Result<Activity, Vec<LineError>>;

fn parse(text: &str, projects: &Projects) -> (Vec<ActivityParseResult>, Vec<ActivityParseResult>) {
    text.lines()
        .enumerate()
        .filter(|(_no, line)| !line.is_empty())
        .map(|(no, line)| (no, ActivityLine::new(line)))
        .map(|(no, activity_line)| (no, activity_line.parse(projects)))
        .map(|(no, activity)| activity.map_err(|e| e.at_line(no)))
        .partition(Result::is_ok)
}

fn combine_errors(errors: Vec<ActivityParseResult>) -> String {
    errors
        .into_iter()
        .map(Result::unwrap_err)
        .flatten()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects_and_tasks::{
        project::ProjectWithTasksBuilder, projects::ProjectsBuilder, task::TaskBuilder,
        tasks::TasksBuilder,
    };

    fn projects() -> Projects {
        ProjectsBuilder::new()
            .with_projects(vec![ProjectWithTasksBuilder::new()
                .with_name("Some specific project".to_string())
                .with_tasks(
                    TasksBuilder::new()
                        .with_tasks(vec![TaskBuilder::new()
                            .with_name("Some specific task".to_string())
                            .build()])
                        .build(),
                )
                .build()])
            .build()
    }

    #[test]
    fn it_gives_errors_for_all_lines() {
        let lines = r#"
 | A2020-01-12T08:00:00 | B2020-01-12T08:30:00 | Bla     | Bla  | Description | 

 | 2020-01-12T08:00:00  | 2020-01-12T08:30:00  | Project | Task | Description | 


 | A2020-01-12T08:00:00 | 2020-01-12T08:30:00  | Project | Bla  | Description | 
"#;

        let errors = validate(lines, &projects());

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

pub fn parse_projects(text: &str) -> Result<Projects, serde_json::Error> {
    let harvest_projects = HarvestProjectAssignments::from(text)?;
    Ok(harvest_projects.to_projects())
}
