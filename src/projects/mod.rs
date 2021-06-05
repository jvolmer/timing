pub mod harvest;

#[derive(Debug, PartialEq)]
pub struct Project {
    id: u32,
    name: String,
    tasks: Tasks
}
    
#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<Project>
}

#[derive(Debug, PartialEq)]
pub struct Task {
    id: u32,
    name: String
}

#[derive(Debug, PartialEq)]
pub struct Tasks {
    tasks: Vec<Task>
}
