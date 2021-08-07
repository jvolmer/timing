use crate::projects::task::{Task, TaskBuilder};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestTask {
    pub task: HarvestTaskIdentification,
}

impl HarvestTask {
    pub fn to_task(self) -> Task {
        TaskBuilder::new()
            .with_id(self.task.id)
            .with_name(self.task.name)
            .build()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct HarvestTaskIdentification {
    pub id: u32,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_task() {
        let harvest_task = HarvestTask {
            task: HarvestTaskIdentification {
                id: 4343,
                name: "task".to_string(),
            },
        };

        let task = harvest_task.to_task();

        assert_eq!(
            task,
            TaskBuilder::new()
                .with_id(4343)
                .with_name("task".to_string())
                .build()
        );
    }
}
