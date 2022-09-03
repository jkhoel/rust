use std::env;
use std::process;

use rustgrep::Config;

// $ cargo run <query string> <file to parse>

fn main() {
    // Read arguments to a vector
    let args: Vec<String> = env::args().collect();

    // Parse the arguments
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Execute the main run() function and handle any errors it might return
    if let Err(e) = rustgrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
