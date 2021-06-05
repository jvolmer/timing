mod project;

use crate::projects::{Project, Projects};
use crate::projects::harvest::project::{HarvestProject};
use serde::{Deserialize};
use serde_json::Result;

#[derive(Deserialize, Debug, PartialEq)]
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
    use crate::projects::{Task, Tasks};

    #[test]
    fn if_parses_json_into_project_list() {
	let json = HARVEST_PROJECTS;
	
	let harvest_projects = HarvestProjectAssignments::from(json).unwrap();
	let projects = harvest_projects.to_projects();

	assert_eq!(
	    projects,
	    Projects{
		projects: vec![
		    Project {
			id: 95783638,
			name: "Buddy".to_string(),
			tasks: Tasks {
			    tasks: vec![
				Task {
				    id: 42180014,
				    name: "Buddy (name buddy)".to_string()
				}
			    ]
			}
		    }		    
		],
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
