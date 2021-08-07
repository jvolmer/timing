use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LineError(usize, String);

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{} | {}", self.0, self.1)
    }
}

impl LineError {
    pub fn new(line_number: usize, message: String) -> Self {
	Self (line_number, message)
    }
}
    
