use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let string = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &string)
    } else {
        search(&config.query, &string)
    };
    for line in results.iter() {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config{
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "DucT";
        let contents = "\
Rust:
safe, fast, productive.
pick three";
        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}