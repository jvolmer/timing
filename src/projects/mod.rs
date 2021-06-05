pub mod harvest;

#[derive(Debug, PartialEq)]
pub struct Project {
    id: u32,
    name: String
}
    
#[derive(Debug, PartialEq)]
pub struct Projects {
    projects: Vec<Project>
}
}
