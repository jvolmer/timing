use crate::projects::project::{Project, ProjectBuilder};
use crate::projects::task::Task;
use crate::projects::tasks::TasksBuilder;
use crate::projects::harvest::task::HarvestTask;
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

	ProjectBuilder::new()
	    .with_name(self.project.name)
	    .with_id(self.project.id)
	    .with_tasks(TasksBuilder::new()
			.with_tasks(tasks)
			.build())
	    .build()
    }
}
    
#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestProjectIdentification {
    pub id: u32,
    pub name: String,
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::harvest::task::{HarvestTask, HarvestTaskIdentification};
    use crate::projects::task::TaskBuilder;
    	
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
	    ProjectBuilder::new()
		.with_id(1234)
		.with_name("project".to_string())
		.with_tasks(TasksBuilder::new()
			    .with_tasks(vec![
				TaskBuilder::new()
				    .with_id(999)
				    .with_name("task".to_string())
				    .build()])
			    .build())
		.build()
	);
    }
}