use crate::projects::tasks::{Tasks, TasksBuilder};
use crate::projects::named::Named;
use crate::projects::task::Task;
use crate::projects::project_error::ProjectError;
use crate::projects::list_with_names::ListWithNames;

#[derive(Debug, PartialEq, Clone)]
pub struct ProjectWithTasks {
    id: u32,
    name: String,
    tasks: Tasks
}

impl Named for ProjectWithTasks {
    fn name(&self) -> &str {
	&self.name
    }
}

impl ProjectWithTasks {
    pub fn find_task(&self, search_string: &str) -> Result<&Task, ProjectError> {
	self.tasks.find(search_string)
	    .map_err(|err| ProjectError::Task(err))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::task::TaskBuilder;

    #[test]
    fn it_finds_task_with_specific_string() {
	let expected_task = TaskBuilder::new()
	    .with_name("Task with some speciality".to_string())
	    .build();
	let project = ProjectWithTasksBuilder::new()
	    .with_tasks(TasksBuilder::new()
			.with_tasks(vec![
			    expected_task.clone()])
			.build())
	    .build();
	
	let task = project.find_task("Special");

	assert_eq!(task, Ok(&expected_task));
    }
}


pub struct ProjectWithTasksBuilder {
    id: u32,
    name: String,
    tasks: Tasks
}

impl ProjectWithTasksBuilder {
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
    
    pub fn build(self) -> ProjectWithTasks {
	ProjectWithTasks {
	    id: self.id,
	    name: self.name,
	    tasks: self.tasks
	}
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Project {
    id: u32,
    name: String,
}

impl Project {
    pub fn new(project_with_tasks: &ProjectWithTasks) -> Self {
	Self {
	    id: project_with_tasks.id.clone(),
	    name: project_with_tasks.name.clone()
	}
    }
}

pub struct ProjectBuilder {
    id: u32,
    name: String,
}

impl ProjectBuilder {
    pub fn new() -> Self {
	Self {
	    id: 1,
	    name: "project".to_string(),
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
    
    pub fn build(self) -> Project {
	Project {
	    id: self.id,
	    name: self.name,
	}
    }
}
