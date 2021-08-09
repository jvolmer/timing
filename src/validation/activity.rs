use crate::projects_and_tasks::{project::Project, task::Task};
use crate::validation::time::{End, Start};

#[derive(Debug, PartialEq)]
pub struct Description {
    text: String,
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
    description: Description,
}

impl Activity {
    pub fn from(
        start: Start,
        end: End,
        project: Project,
        task: Task,
        description: Description,
    ) -> Self {
        Self {
            start,
            end,
            project,
            task,
            description,
        }
    }
}
