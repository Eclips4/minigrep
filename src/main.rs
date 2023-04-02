mod search;
mod base;
use std::{env, fs};
use std::process;
use base::Config;
use search::search;


fn run(config: Config) -> () {
    let contents = fs::read_to_string(config.file_path).unwrap();
    let results = search(&config.query, &contents, config.ignore_case);
    match results {
        Some(lines) => {
            for line in lines {
                println!("{}, line: {}", line.string.trim(), line.line_number)
            }
        },
        None => println!("No string '{}' found", config.query)
    }
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config);
}

