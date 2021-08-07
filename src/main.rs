use std::{env, process};
use timing::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let timings = std::fs::read_to_string(&config.timing_file).unwrap_or_else(|err| {
        eprintln!("Timings: {}", err);
        process::exit(1);
    });

    let projects_string = std::fs::read_to_string(&config.projects_file).unwrap_or_else(|err| {
        eprintln!("Projects: {}", err);
        process::exit(1);
    });

    let projects = timing::parse_projects(&projects_string).unwrap_or_else(|err| {
        eprintln!("Problem parsing projects file: {}", err);
        process::exit(1);
    });

    println!("{}", timing::validate(&timings, &projects));
}
