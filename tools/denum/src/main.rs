use clap::{arg, command};
use denum::*;

fn main() {
    // Define program args and options
    let cli_args = command!()
        // Required args
        .arg(arg!(-l --list <WORDLIST> "Path to wordlist"))
        // Input Options
        .arg(arg!(-a --address <HOSTADDRESS> "Ommit to target local paths").required(false))
        // Filtering Options
        .arg(arg!(-s --status <STATUSCODE> "Limit output to this status code").required(false)) // TODO
        // Output Options
        .arg(arg!(-o --output <FILENAME> "Outputs to file").required(false)) // TODO
        .arg(arg!(--json "Outputs to JSON format").required(false)) // TODO
        .arg(arg!(--csv "Outputs to CSV format").required(false)) // TODO
        .get_matches();

    // TODO:    It would be cool to make an option to have the list-path point to the root of a danielmiessler/SecLists
    //          repository, and then use all lists found inside the `discovery` folder:
    //          - Web-Content/combined_directories.txt perhaps if --address is set

    // Get the wordlist path from program args
    let wordlist_path = cli_args
        .get_one::<String>("list")
        .expect("Unable to read wordlist path");

    println!("==> Using Wordlist: {}", wordlist_path);

    // Parse the wordlist into QueryObjects
    let _query_objects = digest_wordlist(wordlist_path).expect("Unable to parse the wordlist");
}
