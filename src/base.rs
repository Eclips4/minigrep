use std::env;

pub struct Information<'a> {
    pub string: &'a str,
    pub line_number: u32,
}


pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

const DEFAULT_PATH: &str = "src/file.txt";

fn no_file_path() -> String {
    println!("No file path param detected. \nSet file path to {}", DEFAULT_PATH);
    String::from(DEFAULT_PATH)
}

impl Config{
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => no_file_path()
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(
            Self{
                query,
                file_path,
                ignore_case
            }
        )
    }

}

pub type Search<'a> =  Option<Vec<Information<'a>>>;
