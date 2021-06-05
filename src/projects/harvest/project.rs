use crate::projects::{Project};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HarvestProject {
    pub project: HarvestProjectIdentification,
}
    
impl HarvestProject {
    pub fn to_project(self) -> Project {
	Project {
	    id: self.project.id,
	    name: self.project.name
	}
    }
}
    
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HarvestProjectIdentification {
    pub id: u32,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    	
    #[test]
    fn it_creates_project() {
	let harvest_project = HarvestProject {
	    project: HarvestProjectIdentification {
		id: 1234,
		name: "project".to_string()
	    }
	};
	    
	let project = harvest_project.to_project();
	
	assert_eq!(
	    project,
	    Project {
		id: 1234,
		name: "project".to_string()
	    }
	);
    }
}
