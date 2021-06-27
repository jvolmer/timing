use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ProjectError {
    from: String,
    message: String
}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}: {}", self.from, self.message)
    }
}

impl ProjectError {
    pub fn new(message: &str) -> Self {
	Self {
	    from: "timing".to_string(),
	    message: message.to_string()
	}
    }
}
