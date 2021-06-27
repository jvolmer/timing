use crate::projects::{project::Project, task::Task};
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Activity {
    start: DateTime<Local>,
    end: DateTime<Local>,
    project: Project,
    task: Task,
    description: String
}

impl Activity {
    pub fn from(
	start: DateTime<Local>,
	end: DateTime<Local>,
	project: Project,
	task: Task,
	description: String
    ) -> Self {
	Self { start, end, project, task, description }
    }
}
