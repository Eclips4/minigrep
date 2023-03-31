use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}


const DEFAULT_PATH: &str = "src/file.txt";

fn no_file_path() -> String {
    println!(
        "No file_path param detected.\
        Sets file_path to: {DEFAULT_PATH}"
    );
    return String::from(DEFAULT_PATH);
}


impl Config {
    pub fn
    build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str>
    {
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
        Ok(Self{
            query,
            file_path,
            ignore_case
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}")
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query_lower_case = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lower_case))
        .collect()
}

pub fn insensitive_first_word_match<'a>(query: &str, contents: &'a str) -> &'a str {
    let query = query.to_lowercase();
    let find = contents.clone().to_lowercase().find(&query);
    match find {
        Some(start_index) => &contents[start_index..(start_index + query.len())],
        None => &"-1"
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn find_exist_word_insensitive(){
        let query = "RUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!("Rust", insensitive_first_word_match(query, contents));
    }

    #[test]
    fn find_non_exist_word_insensitive(){
        let query = "Trusty";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!("-1", insensitive_first_word_match(query, contents));
    }
}