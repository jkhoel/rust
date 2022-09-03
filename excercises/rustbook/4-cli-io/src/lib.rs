use std::error::Error;
use std::fs;

// TODO: Continue from 12.5 in the Rust Book

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters! Make sure to pass a query-string and a file-path");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// The ideomatic way of writing code in Rust is that `main` handles
// configuration and setting up - the rest happens in a run function:
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // the `?`will return the error value from the current function

    println!("----");

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    // Using () like this is the idiomatic way to indicate that we’re calling
    // run for its side effects only; it doesn’t return a value we need.
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";

        // Note: The rust-book example will fail since vscode injects spaces. Using new-line delimiters instead
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        // We assert that the result of search() only contains the line with `duct` in it:
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
