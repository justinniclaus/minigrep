use std::error::Error;  // Error trait for returning errors from run function.
use std::fs;            // File system operations (reading the file).
use std::env;           // Check environment variables to see if IGNORE_CASE is set.

pub struct Config 
{
    pub query: String,      // The text to search for.
    pub file_path: String,  // The file where we will search for the query.
    pub ignore_case: bool,  // Flag indicating whether to ignore case when searching.
}

impl Config
{
    // Builds a Config object from a slice of string arguments.
    // Returns Ok(Config) if successful, or an Err(&'static str) if validation fails.
    pub fn build(args: &[String]) -> Result<Config, &'static str> 
    {   
        // We need at least 3 arguments: the binary name, the query, and the file path.
        if args.len() < 3
        {
            return Err("Not enough arguments!");
        }

        // Extract the query and file path from the arguments.
        let query = args[1].clone();
        let file_path = args[2].clone();

        // Check if the IGNORE_CASE environment variable is set.
        let ignore_case = env::var("IGNORE_CASE").is_ok();


        // Return a new Config instance using the extracted information.
        Ok(Config 
        {
            query, 
            file_path,
            ignore_case,
        })
    }
}

// The main logic of the program: reading the file and searching for the query.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    // Read the entire file into a single string.
    let contents = fs::read_to_string(config.file_path)?;

    // Depending on whether ignore_case is true, choose the appropriate search function.
    let results = if config.ignore_case
    {
        search_case_insensitive(&config.query, &contents)
    }
    else
    {
        search(&config.query, &contents)
    };

    // Print each line that matches the query.
    for line in results 
    {
        println!("{}", line);
    }

    // Return Ok if everything completed successfully.
    Ok(())
}

// Searches for exact matches (case sensitive).
// Returns a vector of string slices that contain the query.
pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> 
{
    let mut results = Vec::new();

    // Iterate over each line in the contents and check if it contains the query.
    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }
    results
}

// Searches for matches ignoring case differences.
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> 
{
    // Convert the query to lowercase.
    let query = query.to_lowercase();
    let mut results = Vec::new();

    // Iterate over each line, convert to lowercase, then check for the query.
    for line in contents.lines() 
    {
        if line.to_lowercase().contains(&query) 
        {
            results.push(line);
        }
    }
    results
}

// Unit tests to ensure that the functions behave as expected.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        // We expect one result for an exact match of 'duct' (ignoring "Duct Tape.").
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three.
Trust me.";

        // We expect both "Rust:" and "Trust me." to match, ignoring case differences.
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn one_result() 
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        // Another test to confirm that we match only the line that contains 'duct'.
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
