# Timeval [![Build](https://github.com/jvolmer/timing/actions/workflows/build.yml/badge.svg)](https://github.com/jvolmer/timing/actions/workflows/build.yml) [![Code Coverage](https://codecov.io/gh/jvolmer/timing/branch/main/graph/badge.svg)][codecov]

> Validate your timesheet

## Background

Do you want to make sure that your timesheet respects a set of given rules without scanning the timesheet manually? Then this CLI is for you: Write your timesheet in a simple text file, run the tool and get a list of errors back.

A timesheet is a collection of time-entries. Each time-entry is a time duration where you work on a task of a project.

### Validation rules

This CLI validates a given timesheet based on specific validation rules. I plan to implement these validations:

- [x] The input includes correctly formatted content
- [x] Both project and task of an entry are known
- [ ] Entries don't overlap in time
- [ ] The description of an entry includes a ticket number
- [ ] Breaks are taken after specific work durations

This is work in progress, I'll mark a validation as soon as it is available.

## Usage

You need to [install rust][rust-install]. Clone this directory. Inside you exectute

```shell
cargo run <timesheet-file> --projects <projects-file>
```

### Help

Get more information on the CLI options via

```bash
cargo run -- --help
```

### Tests

Run all the test via 

```bash
cargo test
```

Test coverage is available [here][codecov].

### Example

1. Write your timesheet in a textfile. In this file, each line includes a time-entry in the format

```org
| <start> | <end> | <project> | <task> | <description> |
``` 

This is an example (`example/timesheet.org`):

```org
| 2020-01-12T08:00:00 | 2020-01-12T08:30:00 | Timeval  | Build      | Improve Readme file                    |
| 2020-01-12T08:30:00 | 2020-01-12T10:30:00 | Timeval  | Build      | Create new error                       |
| a wrong time format | 2020-01-12T12:30:00 | Training | Self-Study | Read about Readme files best practices |
```

2. Define your valid projecs and tasks in a json file. This is a minimal example (`example/projects.json`):

```json
{
    "project_assignments": [
        {
            "project": {
                "id": 1,
                "name": "My Timeval project"
            },
            "task_assignments": [
                {
                    "task": {
                        "id": 1,
                        "name": "Build stuff"
                    }
                },
                {
                    "task": {
                        "id": 2,
                        "name": "Do research"
                    }
                },
                {
                    "task": {
                        "id": 3,
                        "name": "Planning"
                    }
                }
            ]
        },
        {
            "project": {
                "id": 2,
                "name": "Training"
            },
            "task_assignments": [
                {
                    "task": {
                        "id": 4,
                        "name": "Self-study"
                    }
                }
            ]
        }
    ]
}
```

3. Run the command

```shell
cargo run example/timesheet.org -p example/projects.json
```

This returns with
```shell
|   2 | Start      | ParseError: String is not convertable to date time                  |
```
This output includes information about the line in the timesheet file at which an error occured (here line 2), the erroneous item (here start), and an error description.

## Install

You need to [install rust][rust-install]. Clone this repository. Inside execute
```bash
cargo install --path .
```

This creates an executable in your ~/.cargo/bin/ directory, which should already be part of your PATH variable. Now run the CLI with 
```bash
timing --help
```

## Features

- [x] In the timesheet, be able to reference project and task by a substring of their full name. The full name is defined in the projects file.
- [ ] Integrate a specific timesheet API to get project information from the API and push validated time entries to the API. I plan to integrate with [Harvest](https://www.getharvest.com/?hsLang=en) - the current project file format is already based on their API.
- [ ] Create a report: How many hours in total did you work, how many hours per project, ...
- [ ] Choose which validations to use via command line.
- [ ] Add automatic fixes: E.g. Add breaks after appropriate durations and reschedule subsequent entries.


[codecov]: https://codecov.io/gh/jvolmer/timing
[rust-install]: https://www.rust-lang.org/tools/install
