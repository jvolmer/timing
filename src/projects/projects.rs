use crate::projects::project::Project;
use crate::projects::list_with_names::ListWithNames;

#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<Project>
}

impl ListWithNames<Project> for Projects {
    fn items(&self) -> std::slice::Iter<Project> {
	self.projects.iter()
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
