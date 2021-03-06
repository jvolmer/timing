use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ProjectError {
    Project(SearchError),
    Task(SearchError),
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Project(error) => write!(f, "{:<10} | ProjectError: {}", "Project", error),
            Self::Task(error) => write!(f, "{:<10} | ProjectError: {}", "Task", error),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchError {
    NotFound,
    FoundMoreThanOne,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::NotFound => write!(f, "No match was found"),
            Self::FoundMoreThanOne => write!(f, "More than one match was found"),
        }
    }
}
