use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    from: String,
    message: String
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}: {}", self.from, self.message)
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(error: chrono::ParseError) -> Self {
	ParseError {
	    from: "chrono".to_string(),
	    message: error.to_string()
	}
    }
}

impl ParseError {
    pub fn new(message: String) -> Self {
	ParseError {
	    from: "timing".to_string(),
	    message
	}
    }
}
