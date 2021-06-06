use crate::projects::tasks::{Tasks, TasksBuilder};

#[derive(Debug, PartialEq, Clone)]
pub struct Project {
    id: u32,
    name: String,
    tasks: Tasks
}

impl Project {
    pub fn has_name_with(&self, string: &str) -> bool {
	self.name.to_lowercase()
	    .contains(&string.to_lowercase())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn has_name_when_name_includes_given_string() {
	let project = ProjectBuilder::new()
	    .with_name("New project".to_string())
	    .build();

	let has_name = project.has_name_with("new");

	assert_eq!(has_name, true);
    }

    fn do_not_has_name_when_name_does_not_include_given_string() {
	let project = ProjectBuilder::new()
	    .with_name("New project".to_string())
	    .build();

	let has_name = project.has_name_with("abc");

	assert_eq!(has_name, false);
    }


	
//     mod it_searches_for_string_in_task_names_for_specific_project {
// 	use super::*;

// 	#[test]
// 	fn it_finds_one_task() {
// 	}
//     }
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
