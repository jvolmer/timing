use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooFewArguments,
    ArgumentErrors(Vec<ArgumentParseError>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let error_name = "ParseError";
	match &self {
	    Self::TooFewArguments => write!(f, "All | {}: Too few arguments given", error_name),
	    Self::ArgumentErrors(errors) => write!(f, "{:?}", errors)
	}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentParseError {
    Start(DateTimeParseError),
    End(DateTimeParseError),
    ProjectAndTask
}

impl fmt::Display for ArgumentParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match &self {
	    Self::Start(error) => write!(f, "Start | ParseError: {}", error),
	    Self::End(error) => write!(f, "End | ParseError: {}", error),
	    Self::ProjectAndTask => write!(f, "{}", self),
	}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DateTimeParseError {
    NotConvertible
}

impl fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "String is not convertable to date time")
    }
}
