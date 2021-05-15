use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct HarvestProjectAssignments {
    project_assignments: Vec<HarvestProject>,
}
#[derive(Serialize, Deserialize)]
pub struct HarvestProject {
    project: HarvestProjectIdentification,
}
#[derive(Serialize, Deserialize)]
pub struct HarvestProjectIdentification {
    id: u32,
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct Project {
    id: u32,
    name: String
}

#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<Project>,
}
    
impl Projects {
    pub fn from(string: &str) -> Result<Projects> {
	let assignments: HarvestProjectAssignments = serde_json::from_str(string)?;
	let projects: Vec<Project> = assignments.project_assignments
	    .into_iter()
	    .map(|project| Project {
		id: project.project.id,
		name: project.project.name
	    })
	    .collect();
	Ok(Projects {
	    projects: projects
	})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_json_into_object() {
	let json = String::from(HARVEST_PROJECTS);
	assert_eq!(
	    Projects::from(&json).unwrap(),
	    Projects {
		projects: vec![
		    Project {
			id: 95783638,
			name: "lise Buddy".to_string()
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
        "name": "lise Buddy",
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
//    }
}
