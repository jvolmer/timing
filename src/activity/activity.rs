use crate::activity::time::{Start, End};
use crate::projects::{project::Project, task::Task};

#[derive(Debug, PartialEq)]
pub struct Description {
    text: String
}

impl Description {
    pub fn new(text: String) -> Self {
	Self { text }
    }
}


#[derive(Debug, PartialEq)]
pub struct Activity {
    start: Start,
    end: End,
    project: Project,
    task: Task,
    description: Description
}

impl Activity {
    pub fn from(
	start: Start,
	end: End,
	project: Project,
	task: Task,
	description: Description
    ) -> Self {
	Self { start, end, project, task, description }
    }
}
