use clap::{load_yaml, App};
use std::process;
use timing::projects::projects::Projects;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    println!(
        "{}",
        timing::validate(
            &self::timings(matches.value_of("INPUT").unwrap()),
            &self::projects(
                matches
                    .value_of("projects")
                    .unwrap_or("input/projects.json")
            )
        )
    );
}

fn timings(file: &str) -> String {
    std::fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("Timings: {}", err);
        process::exit(1);
    })
}

fn projects(file: &str) -> Projects {
    let content = std::fs::read_to_string(file).unwrap_or_else(|err| {
        eprintln!("Projects: {}", err);
        process::exit(1);
    });

    timing::parse_projects(&content).unwrap_or_else(|err| {
        eprintln!("Problem parsing projects file: {}", err);
        process::exit(1);
    })
}
