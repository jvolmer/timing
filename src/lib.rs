use serde_json::{Result, Value};

#[derive(Debug, PartialEq)]
pub struct Project {
    id: String,
    name: String
}

impl Project {
    pub fn from(string: &str) -> Result<Project> {
	let json: Value = serde_json::from_str(string)?;
	println!("{}", json["project_assignments"][0]["project"]["name"].as_str().unwrap());
	Ok(Project {
	    id: json["project_assignments"][0]["project"]["id"].to_string(),
	    name: json["project_assignments"][0]["project"]["name"].as_str().unwrap().to_string()
	})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_json_into_object() {
	let json = projects_json();
	assert_eq!(
	    Project::from(&json).unwrap(),
	    Project {
		id: "95783638".to_string(),
		name: "lise Buddy".to_string()
	    });
    }

    fn projects_json() -> String {
	return String::from(r#"
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
"#)
    }
}
