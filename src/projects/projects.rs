use crate::projects::project::{Project};

#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<Project>
}

impl Projects {
    fn find(&self, name: &str) -> Result<&Project, &str> {
	let found_projects: Vec<&Project> = self.projects
	    .iter()
	    .filter(|project| project.has_name_with(name))
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
    use crate::projects::project::ProjectBuilder;
    
    mod it_searches_for_string_in_project_names {
	use super::*;
	
	#[test]
	fn it_finds_one_project() {
	    let projects = ProjectsBuilder::new()
		.with_projects(vec![
		    ProjectBuilder::new()
			.with_name("New Project X".to_string())
			.build()])
		.build();

	    let found_project = projects.find("new");
	    
	    assert_eq!(found_project.unwrap(), projects.projects.get(0).unwrap());
	}

	#[test]
	fn errors_when_no_project_is_found() {
	    let projects = ProjectsBuilder::empty().build();

	    let found_project = projects.find("new");

	    assert_eq!(found_project, Err("No project found"));
	}

	#[test]
	fn errors_when_more_than_one_project_is_found() {
	    let projects = ProjectsBuilder::new()
		.with_projects(vec![
		    ProjectBuilder::new()
			.with_name("New Project X".to_string())
			.build(),
		    ProjectBuilder::new()
			.with_name("This is a new project".to_string())
			.build()])
		.build();

	    let found_project = projects.find("new");

	    assert_eq!(found_project, Err("More than one project found"));
	}

    }
}

pub struct ProjectsBuilder {
    projects: Vec<Project>
}

impl ProjectsBuilder {
    pub fn new() -> Self {
	Self::empty()
    }
    
    pub fn empty() -> Self {
	Self {
	    projects: vec![]
	}
    }

    pub fn with_projects(mut self, projects: Vec<Project>) -> Self {
	self.projects = projects;
	self
    }
    
    pub fn build(self) -> Projects {
	Projects {
	    projects: self.projects
	}
    }
}
