use crate::projects::{Project, Task, Tasks};
use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestProject {
    pub project: HarvestProjectIdentification,
    pub task_assignments: Vec<HarvestTask>
}
    
impl HarvestProject {
    pub fn to_project(self) -> Project {
	let tasks: Vec<Task> = self.task_assignments
	    .into_iter()
	    .map(|harvest_task| harvest_task.to_task())
	    .collect();

	Project {
	    id: self.project.id,
	    name: self.project.name,
	    tasks: Tasks { tasks }
	}
    }
}
    
#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestProjectIdentification {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestTask {
    pub task: HarvestTaskIdentification
}

impl HarvestTask {
    pub fn to_task(self) -> Task {
	Task {
	    id: self.task.id,
	    name: self.task.name
	}
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestTaskIdentification {
    pub id: u32,
    pub name: String
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
	    },
	    task_assignments: vec![
		HarvestTask {
		    task: HarvestTaskIdentification{
			id: 999,
			name: "task".to_string()
		    }
		}
	    ]

	};
	    
	let project = harvest_project.to_project();
	
	assert_eq!(
	    project,
	    Project {
		id: 1234,
		name: "project".to_string(),
		tasks: Tasks {
		    tasks: vec![
			Task {
			    id: 999,
			    name: "task".to_string()
			}
		    ]
		}
	    }
	);
    }

    #[test]
    fn it_creates_task() {
	let harvest_task = HarvestTask {
	    task: HarvestTaskIdentification {
		id: 4343,
		name: "task".to_string()
	    }
	};

	let task = harvest_task.to_task();

	assert_eq!(
	    task,
	    Task {
		id: 4343,
		name: "task".to_string()
	    }
	);
    }
}
