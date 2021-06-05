pub mod harvest;

#[derive(Debug, PartialEq, Clone)]
pub struct Project {
    id: u32,
    name: String,
    tasks: Tasks
}
    
#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<Project>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    id: u32,
    name: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tasks {
    tasks: Vec<Task>
}

impl Projects {
    fn find(&self, name: &str) -> Result<&Project, &str> {
	let found_projects: Vec<&Project> = self.projects
	    .iter()
	    .filter(|project| project.name.to_lowercase()
		    .contains(&name.to_lowercase()))
	    .collect();

	match found_projects.len() {
	    1 => Ok(found_projects.get(0).unwrap()),
	    0 => Err("No project found"),
	    _ => Err("More than one project found")
	}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub struct ProjectBuilder {
	id: u32,
	name: String,
	tasks: Tasks
    }

    impl ProjectBuilder {
	fn new() -> ProjectBuilder {
	    ProjectBuilder {
		id: 1,
		name: "project".to_string(),
		tasks: Tasks {
		    tasks: vec![]
		}
	    }
	}

	fn with_name(mut self, name: &str) -> ProjectBuilder {
	    self.name = name.to_string();
	    self
	}
	
	fn build(self) -> Project {
	    Project{
		id: self.id,
		name: self.name,
		tasks: self.tasks
	    }
	}
    }
    
    mod it_searches_for_string_in_project_names {
	use super::*;
	
	#[test]
	fn if_finds_one_project() {
	    let new_project = ProjectBuilder::new()
		.with_name("New Project X")
		.build();
	    let projects = Projects {
		projects: vec![
		    new_project.clone()
		]
	    };

	    let found_project = projects.find("new");
	    
	    assert_eq!(found_project.unwrap(), &new_project);
	}

	#[test]
	fn errors_when_no_project_is_found() {
	    let projects = Projects {
		projects: vec![]
	    };

	    let found_project = projects.find("new");

	    assert_eq!(found_project, Err("No project found"));
	}

	#[test]
	fn errors_when_more_than_one_project_is_found() {
	    let projects = Projects {
		projects: vec![
		    ProjectBuilder::new()
			.with_name("New Project X")
			.build(),
		    ProjectBuilder::new()
			.with_name("This is a new project")
			.build()
		]
	    };

	    let found_project = projects.find("new");

	    assert_eq!(found_project, Err("More than one project found"));
	}

    }
}
