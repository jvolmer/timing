#[derive(Debug, PartialEq, Clone)]
pub struct Task {
    id: u32,
    name: String
}


pub struct TaskBuilder {
    id: u32,
    name: String
}

impl TaskBuilder {
    pub fn new () -> Self {
	Self {
	    id: 1,
	    name: "task".to_string(),
	}
    }

    pub fn with_id(mut self, id:u32) -> Self {
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
	    name: self.name
	}
    }
}
