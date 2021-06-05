mod project;
mod tasks;

use crate::projects::{Projects, Project};
use crate::projects::harvest::project::{HarvestProject, HarvestProjectIdentification};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HarvestProjectAssignments {
    project_assignments: Vec<HarvestProject>,
}

impl HarvestProjectAssignments {
    pub fn from(string: &str) -> Result<HarvestProjectAssignments> {
	serde_json::from_str(string)
    }
	
    pub fn to_projects(self) -> Projects {
	let projects: Vec<Project> = self.project_assignments
	    .into_iter()
	    .map(|harvest_project| harvest_project.to_project())
	    .collect();
	Projects { projects }
    }
}
            
#[cfg(test)]
mod tests {
    use super::*;
    	
    #[test]
    fn it_parses_json_into_harvest_project_assignments() {
	let json = HARVEST_PROJECTS;
	
	let object = HarvestProjectAssignments::from(json).unwrap();
	    
	assert_eq!(
	    object,
	    HarvestProjectAssignments {
		project_assignments: vec![
		    HarvestProject {
			project: HarvestProjectIdentification {
			    id: 95783638,
			    name: "Buddy".to_string()
			}
		    }
		]
	    }
	);
    }

    #[test]
    fn it_creates_project_list() {
	let harvest_project_assignements = HarvestProjectAssignments {
	    project_assignments: vec![
		HarvestProject {
		    project: HarvestProjectIdentification {
			id: 1234,
			name: "project".to_string()
		    }
		}
	    ]
	};
	    
	let projects = harvest_project_assignements.to_projects();
	    
	assert_eq!(
	    projects,
	    Projects{
		projects: vec![
		    Project {
			id: 1234,
			name: "project".to_string()
		    }		    
		]
	    }
	);
    }
	
    const HARVEST_PROJECTS: &str = r#"
{
  "project_assignments": [
    {
      "id": 123453221,
      "is_project_manager": false,
      "is_active": true,
      "use_default_rates": true,
      "budget": null,
      "created_at": "2021-03-23T10:39:30Z",
      "updated_at": "2021-03-23T10:39:30Z",
      "hourly_rate": null,
      "project": {
        "id": 95783638,
        "name": "Buddy",
        "code": "buddy",
        "is_billable": true
      },
      "client": {
        "id": 434566,
        "name": "internal projects",
        "currency": "EUR"
      },
      "task_assignments": [
        {
          "id": 137838821,
          "billable": false,
          "is_active": true,
          "created_at": "2019-08-02T11:40:33Z",
          "updated_at": "2019-08-02T11:40:33Z",
          "hourly_rate": null,
          "budget": null,
          "task": {
            "id": 42180014,
            "name": "Buddy (name buddy)"
          }
        }
      ]
    }
  ]
}
"#;
}
