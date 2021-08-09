use crate::activity::activity::Activity;
use crate::line_error::LineError;
use crate::parser::activity_line::ActivityLine;
use crate::projects::harvest::projects::HarvestProjectAssignments;
use crate::projects::projects::Projects;

mod activity;
mod line_error;
mod parser;
pub mod projects;

pub fn validate(text: &str, projects: &Projects) -> String {
    let (_activities, errors) = parse(text, projects);
    combine_errors(errors)
}

fn parse(
    text: &str,
    projects: &Projects,
) -> (
    Vec<Result<Activity, Vec<LineError>>>,
    Vec<Result<Activity, Vec<LineError>>>,
) {
    text.lines()
        .enumerate()
        .filter(|(_no, line)| line.to_string() != "".to_string())
        .map(|(no, line)| (no, ActivityLine::new(line)))
        .map(|(no, activity_line)| (no, activity_line.parse(&projects)))
        .map(|(no, activity)| activity.map_err(|e| e.at_line(no)))
        .partition(Result::is_ok)
}

fn combine_errors(errors: Vec<Result<Activity, Vec<LineError>>>) -> String {
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
    use crate::projects::project::ProjectWithTasksBuilder;
    use crate::projects::projects::ProjectsBuilder;
    use crate::projects::task::TaskBuilder;
    use crate::projects::tasks::TasksBuilder;

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
