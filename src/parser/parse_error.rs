use crate::projects::project_error::ProjectError;
use crate::projects::task::Task;
use crate::projects::project::Project;
use chrono::Local;
use chrono::DateTime;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TooFewArguments,
    ArgumentErrors(Vec<ArgumentParseError>),
    None,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let error_name = "ParseError";
	match &self {
	    Self::TooFewArguments => write!(f, "All | {}: Too few arguments given", error_name),
	    Self::ArgumentErrors(errors) => write!(f, "{:?}", errors),
	    Self::None => write!(f, "No errors found"),
	}
    }
}

#[derive(Clone, Debug)]
enum Argument {
    DateTime(DateTime<Local>),
    ProjectAndTask((Project, Task)),
}

impl ParseError {
    pub fn from_arguments(
	start: &Result<DateTime<Local>, DateTimeParseError>,
	end: &Result<DateTime<Local>, DateTimeParseError>,
	project_and_task: &Result<(Project, Task), ProjectError>
    ) -> Result<(), ParseError> {
	let vec = vec![
	    Self::map_start(start),
	    Self::map_end(end),
	    Self::map_project_and_tak(project_and_task)
	];
	
	let errors = vec.into_iter()
	    .filter(|option| option.is_err())	 
	    .map(|option| option.unwrap_err())
	    .collect::<Vec<ArgumentParseError>>();


	if !errors.is_empty() {
	    return Err(ParseError::ArgumentErrors(errors))
	}

	Ok(())
    }

    fn map_start(start: &Result<DateTime<Local>, DateTimeParseError>) -> Result<Argument, ArgumentParseError> {
	start.clone()
	    .map(|start| Argument::DateTime(start))
	    .map_err(|err| ArgumentParseError::Start(err))
    }

    fn map_end(end: &Result<DateTime<Local>, DateTimeParseError>) -> Result<Argument, ArgumentParseError> {
	end.clone()
	    .map(|end| Argument::DateTime(end))
	    .map_err(|err| ArgumentParseError::End(err))
    }

    fn map_project_and_tak(pt: &Result<(Project, Task), ProjectError>) -> Result<Argument, ArgumentParseError> {
	pt.clone()
	    .map(|pt| Argument::ProjectAndTask(pt))
	    .map_err(|_| ArgumentParseError::ProjectAndTask)
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


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use crate::projects::project::ProjectBuilder;
    use crate::projects::task::TaskBuilder;
    use crate::projects::project_error::SearchError;

    #[test]
    fn it_collects_no_error_when_all_arguments_are_ok() {
	let start = Result::Ok(Local.ymd(2020, 1, 12).and_hms(8, 0, 0));
	let end = Result::Ok(Local.ymd(2020, 1, 12).and_hms(8, 0, 0));
	let project_and_task = Result::Ok((
	    ProjectBuilder::new().build(),
	    TaskBuilder::new().build()
	));
	
	let error = ParseError::from_arguments(&start, &end, &project_and_task);

	assert_eq!(error, Ok(()));
    }

    #[test]
    fn it_collects_all_errors() {
	let start = Result::Err(DateTimeParseError::NotConvertible);
	let end = Result::Err(DateTimeParseError::NotConvertible);
	let project_and_task = Result::Err(ProjectError::Project(SearchError::NotFound));
	
	let error = ParseError::from_arguments(&start, &end, &project_and_task);

	assert_eq!(error, Err(ParseError::ArgumentErrors(vec![
	    ArgumentParseError::Start(DateTimeParseError::NotConvertible),
	    ArgumentParseError::End(DateTimeParseError::NotConvertible),
	    ArgumentParseError::ProjectAndTask,
	])));
    }

}
