use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{Config, run, search};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

