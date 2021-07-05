use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotConvertableToDateTime,
    StartNotConvertableToDateTime(String),
    EndNotConvertableToDateTime(String),
    TooFewArguments
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let error_name = "ParseError";
	match &self {
	    Self::NotConvertableToDateTime => write!(f, "{}: String is not convertable to date time", error_name),
	    Self::StartNotConvertableToDateTime(message) => write!(f, "Start | {}: {}", error_name, message),
	    Self::EndNotConvertableToDateTime(message) => write!(f, "End | {}: {}", error_name, message),
	    Self::TooFewArguments => write!(f, "All | {}: Too few arguments given", error_name)
	}
    }
}
