use crate::activity::time::{End, Start};
use crate::projects::project::Project;
use crate::projects::project_error::ProjectError;
use crate::projects::task::Task;
use crate::LineError;
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
            Self::ArgumentErrors(errors) => write!(
                f,
                "{}",
                errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        }
    }
}

impl ParseError {
    pub fn from_arguments(
        start: &Result<Start, DateTimeParseError>,
        end: &Result<End, DateTimeParseError>,
        project_and_task: &Result<(Project, Task), ProjectError>,
    ) -> Result<(), ParseError> {
        let vec = vec![
            Self::map_start(start),
            Self::map_end(end),
            Self::map_project_and_tak(project_and_task),
        ];

        let errors = vec
            .into_iter()
            .filter(|option| option.is_err())
            .map(|option| option.unwrap_err())
            .collect::<Vec<ArgumentParseError>>();

        if !errors.is_empty() {
            return Err(ParseError::ArgumentErrors(errors));
        }

        Ok(())
    }

    pub fn at_line(self, line_number: usize) -> Vec<LineError> {
        match &self {
            Self::TooFewArguments => vec![LineError::new(line_number, self.to_string())],
            Self::ArgumentErrors(errors) => errors
                .iter()
                .map(|e| LineError::new(line_number, e.to_string()))
                .collect::<Vec<_>>(),
        }
    }

    fn map_start(
        start: &Result<Start, DateTimeParseError>,
    ) -> Result<Argument, ArgumentParseError> {
        start
            .clone()
            .map(Argument::Start)
            .map_err(ArgumentParseError::Start)
    }

    fn map_end(end: &Result<End, DateTimeParseError>) -> Result<Argument, ArgumentParseError> {
        end.clone()
            .map(Argument::End)
            .map_err(ArgumentParseError::End)
    }

    fn map_project_and_tak(
        pt: &Result<(Project, Task), ProjectError>,
    ) -> Result<Argument, ArgumentParseError> {
        pt.clone()
            .map(Argument::ProjectAndTask)
            .map_err(ArgumentParseError::ProjectAndTask)
    }
}

#[derive(Clone, Debug)]
enum Argument {
    Start(Start),
    End(End),
    ProjectAndTask((Project, Task)),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgumentParseError {
    Start(DateTimeParseError),
    End(DateTimeParseError),
    ProjectAndTask(ProjectError),
}

impl fmt::Display for ArgumentParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Start(error) => write!(f, "Start | ParseError: {}", error),
            Self::End(error) => write!(f, "End | ParseError: {}", error),
            Self::ProjectAndTask(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DateTimeParseError {
    NotConvertible,
}

impl fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String is not convertable to date time")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::project::ProjectBuilder;
    use crate::projects::project_error::SearchError;
    use crate::projects::task::TaskBuilder;
    use crate::LineError;
    use chrono::prelude::*;

    #[test]
    fn it_collects_no_error_when_all_arguments_are_ok() {
        let start = Result::Ok(Start::new(Local.ymd(2020, 1, 12).and_hms(8, 0, 0)));
        let end = Result::Ok(End::new(Local.ymd(2020, 1, 12).and_hms(8, 0, 0)));
        let project_and_task =
            Result::Ok((ProjectBuilder::new().build(), TaskBuilder::new().build()));

        let error = ParseError::from_arguments(&start, &end, &project_and_task);

        assert_eq!(error, Ok(()));
    }

    #[test]
    fn it_collects_all_errors() {
        let start = Result::Err(DateTimeParseError::NotConvertible);
        let end = Result::Err(DateTimeParseError::NotConvertible);
        let project_and_task = Result::Err(ProjectError::Project(SearchError::NotFound));

        let error = ParseError::from_arguments(&start, &end, &project_and_task);

        assert_eq!(
            error,
            Err(ParseError::ArgumentErrors(vec![
                ArgumentParseError::Start(DateTimeParseError::NotConvertible),
                ArgumentParseError::End(DateTimeParseError::NotConvertible),
                ArgumentParseError::ProjectAndTask(ProjectError::Project(SearchError::NotFound)),
            ]))
        );
    }

    mod it_resolves_to_vector_of_line_errors {
        use super::*;

        #[test]
        fn for_argument_errors() {
            let error = ParseError::ArgumentErrors(vec![
                ArgumentParseError::Start(DateTimeParseError::NotConvertible),
                ArgumentParseError::End(DateTimeParseError::NotConvertible),
            ]);

            let line_errors = error.at_line(5);

            assert_eq!(
                line_errors,
                vec![
                    LineError::new(
                        5,
                        ArgumentParseError::Start(DateTimeParseError::NotConvertible).to_string()
                    ),
                    LineError::new(
                        5,
                        ArgumentParseError::End(DateTimeParseError::NotConvertible).to_string()
                    ),
                ]
            );
        }

        #[test]
        fn for_too_few_arguments_error() {
            let error = ParseError::TooFewArguments;

            let line_errors = error.at_line(4);

            assert_eq!(
                line_errors,
                vec![LineError::new(4, ParseError::TooFewArguments.to_string())]
            );
        }
    }
}
