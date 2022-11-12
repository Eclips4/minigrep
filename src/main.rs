use std::env;
use std::process;
use minigrep::{Config, run};


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

