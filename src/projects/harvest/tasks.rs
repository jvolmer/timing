use crate::projects::{Task, Tasks};
use serde::{Deserialize};

#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestTaskAssignments {
    pub task_assignments: Vec<HarvestTask>
}

impl HarvestTaskAssignments {
    pub fn to_tasks(self) -> Tasks {
	let tasks: Vec<Task> = self.task_assignments
	    .into_iter()
	    .map(|harvest_task| harvest_task.to_task())
	    .collect();
	Tasks { tasks }	
    }
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

    #[test]
    fn it_creates_task_list() {
	let harvest_task_assignments = HarvestTaskAssignments {
	    task_assignments: vec![
		HarvestTask {
		    task: HarvestTaskIdentification{
			id: 999,
			name: "task".to_string()
		    }
		}
	    ]
	};

	let tasks = harvest_task_assignments.to_tasks();

	assert_eq!(
	    tasks,
	    Tasks {
		tasks: vec![
		    Task {
			id: 999,
			name: "task".to_string()
		    }
		]
	    }
	);
    }
}
