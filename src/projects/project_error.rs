use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ProjectError {
    Project(SearchError),
    Task(SearchError)
}

#[derive(Debug, PartialEq)]
pub enum SearchError {
    NotFound,
    FoundMoreThanOne
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match &self {
	    Self::NotFound => write!(f, "No match was found"),
	    Self::FoundMoreThanOne => write!(f, "More than one match was found"),
	}
    }
}
