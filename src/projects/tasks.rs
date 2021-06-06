use crate::projects::task::Task;

#[derive(Debug, PartialEq, Clone)]
pub struct Tasks {
    tasks: Vec<Task>
}


pub struct TasksBuilder {
    tasks: Vec<Task>
}

impl TasksBuilder {
    pub fn new() -> Self {
	Self::empty()
    }

    pub fn empty() -> Self {
	Self {
	    tasks: vec![]
	}
    }

    pub fn with_tasks(mut self, tasks: Vec<Task>) -> Self {
	self.tasks = tasks;
	self
    }

    pub fn build(self) -> Tasks {
	Tasks {
	    tasks: self.tasks
	}
    }
}
