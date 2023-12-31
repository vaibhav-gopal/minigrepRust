//Error type to return any error
use std::error::Error;
//open and read files
use std::fs;
//read environment variables
use std::env;

//organizing the config better via a struct
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// passing 
impl Config {
    //build takes ownership of a type with the iterator trait with items of String (this is called trait bounds syntax)
    //  trait bound is actually syntatic sugar for defining a generic type and the trait that the generic type implements
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        //throw away the first argument (aka the name of the binary file)
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
    
        // check if the IGNORE_CASE environment variable is set 
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        //passing ownership of cloned strings to struct
        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

// here since we have two input references we need to specify a lifetime for our output reference
// this function returns a vector of all lines of passed in contents that have some match with query
pub fn search<'a>(
    query: &str,
    contents:&'a str
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase(); 
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

//testing functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        //can't use tabs here because it records it literally
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
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
}