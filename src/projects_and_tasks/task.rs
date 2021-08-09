use crate::projects_and_tasks::named::Named;

#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    id: u32,
    name: String,
}

impl Named for Task {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct TaskBuilder {
    id: u32,
    name: String,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self {
            id: 1,
            name: "task".to_string(),
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn build(self) -> Task {
        Task {
            id: self.id,
            name: self.name,
        }
    }
}
