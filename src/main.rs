use std::{env, process};

fn main() {
    let filename = filename(env::args()).unwrap_or_else(|_err| {
        eprintln!("I need an input file");
        process::exit(1);
    });

    let lines = std::fs::read_to_string(filename).unwrap_or_else(|_err| {
        eprintln!("Cannot read file");
        process::exit(1);
    });

    println!("{}", timing::validate(lines));
}

fn filename(mut args: env::Args) -> Result<String, &'static str> {
    args.next();

    match args.next() {
        Some(arg) => Ok(arg),
        None => return Err("Didn't get a file name"),
    }
}
