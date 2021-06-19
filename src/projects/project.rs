use crate::projects::tasks::{Tasks, TasksBuilder};
use crate::projects::named::Named;

#[derive(Debug, PartialEq, Clone)]
pub struct Project {
    id: u32,
    name: String,
    tasks: Tasks
}

impl Named for Project {
    fn name(&self) -> &str {
	&self.name
    }
}


pub struct ProjectBuilder {
    id: u32,
    name: String,
    tasks: Tasks
}

impl ProjectBuilder {
    pub fn new() -> Self {
	Self {
	    id: 1,
	    name: "project".to_string(),
	    tasks: TasksBuilder::empty().build()
	}
    }
    
    pub fn with_name(mut self, name: String) -> Self {
	self.name = name;
	self
    }

    pub fn with_id(mut self, id: u32) -> Self {
	self.id = id;
	self
    }

    pub fn with_tasks(mut self, tasks: Tasks) -> Self {
	self.tasks = tasks;
	self
    }
    
    pub fn build(self) -> Project {
	Project{
	    id: self.id,
	    name: self.name,
	    tasks: self.tasks
	}
    }
}
