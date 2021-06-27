use crate::activity::activity::Activity;
use crate::projects::{projects::Projects, project_error::ProjectError};
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct ParsedActivity {
    start: DateTime<Local>,
    end: DateTime<Local>,
    project: String,
    task: String,
    description: String
}

impl ParsedActivity {
    pub fn from(
	start: DateTime<Local>,
	end: DateTime<Local>,
	project: String,
	task: String,
	description: String
    ) -> Self { Self { start, end, project, task, description }
    }

    fn include(self, projects: &Projects) -> Result<Activity, ProjectError> {
	let (project, task) = projects.get_project_with_task(&self.project, &self.task)?;
	Ok (
	    Activity::from(
		self.start,
		self.end,
		project,
		task,
		self.description
	    ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{
	projects::ProjectsBuilder,
	project::{ProjectWithTasksBuilder, Project},
	task::TaskBuilder,
	tasks::TasksBuilder
    };

    #[test]
    fn it_finds_suitable_project_and_task_in_projects() {
	let parsed_activity = ParsedActivity {
	    start: Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    end: Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    project: "specific project".to_string(),
	    task: "specific task".to_string(),
	    description: "some description".to_string()
	};
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

	let activity = parsed_activity.include(&projects);

	assert_eq!(activity, Ok(Activity::from(
	    Local.ymd(2020, 1, 12).and_hms(8, 0, 0),
	    Local.ymd(2020, 1, 12).and_hms(8, 30, 0),
	    Project::new(&project_to_be_found),
	    task_to_be_found,
	    "some description".to_string())));
    }
}
