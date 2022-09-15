use std::io::{BufRead, BufReader, Lines, Result as IOResult};
use std::{fs::File, path::Path};

// STRUCTS

/**
 * QueryObject:
 * This struct will represent a path that we will enumerate
 */
pub struct QueryObject {
    pub path: String,
    pub status: u8,
}

impl QueryObject {
    // Instanciates one signel QueryObjects
    pub fn new(path: &String) -> Result<QueryObject, &'static str> {
        if !path.is_empty() {
            return Err("Unable to read path from file");
        }

        Ok(QueryObject {
            path: path.clone(),
            status: 0,
        })
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status
    }
}

// FUNCTIONS

/**
 * Reads a file and returns a Lines-iterator
 */
pub fn read_lines_from_file<P>(filename: P) -> IOResult<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

/**
 * Parses a wordlist into a Vector of QueryObjects
 */
pub fn digest_wordlist(path: &String) -> Result<Vec<QueryObject>, &'static str> {
    if path.is_empty() {
        return Err("Missing list argument");
    }

    // Read in paths from the wordlist
    let paths = read_lines_from_file(&path).expect("Unable to read lines from wordlist");

    // Define a vector to hold our paths as QueryObjects, and populate it
    let mut vec: Vec<QueryObject> = vec![];
    for path in paths {
        if let Ok(p) = path {
            // Filter out any strings that are actually comments instead (starts with #)
            if !p.starts_with("#") {
                vec.push(QueryObject { path: p, status: 0 });
            }
        }
    }

    Ok(vec)
}
