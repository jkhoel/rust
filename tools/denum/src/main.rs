use std::sync::{Arc, Mutex};

use clap::{arg, command};
use denum::{concurrency::ThreadPool, *};

const THREAD_POOL_SIZE: usize = 12; // TODO: Might need to lower this number as this can quickly lead to inadvertent DoS attacks

fn main() {
    // Define program args and options
    let cli_args = command!()
        // Required args
        .arg(arg!(-l --list <WORDLIST> "Path to wordlist"))
        // Input Options
        .arg(arg!(-a --address <HOSTADDRESS> "Ommit to target local paths").required(false))
        .arg(
            arg!(-t --threads <THREAD_POOL_SIZE> "Number of threads to spawn (defaults is 12)")
                .required(false),
        )
        // Filtering Options
        .arg(arg!(-s --status <STATUSCODE> "Limit output to this status code").required(false)) // TODO
        // Output Options
        .arg(arg!(-m --mute <BOOL> "Mute everything but results (for piping)").required(false)) // TODO and make into flag
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
    let query_objects = digest_wordlist(wordlist_path).expect("Unable to parse the wordlist");

    println!("    > Found {} paths", query_objects.len());

    // Create the thread-pool
    let pool_size = match cli_args.get_one::<String>("threads") {
        Some(x) => string_to_usize(x).expect("Invalid threads argument"),
        _ => THREAD_POOL_SIZE,
    };

    println!("==> Using Thread Pool Size: {}", pool_size);

    let thread_pool = ThreadPool::new(pool_size).expect("Failed to create thread-pool");

    // Create mutable references for shared thread values
    let queue = Arc::new(Mutex::new(query_objects));

    // Start looping trough the queue
    loop {
        let local_queue = Arc::clone(&queue);

        if local_queue.lock().unwrap().len() <= 0 {
            break;
        }

        thread_pool.execute(move |_id| {
            let mut thread_queue = local_queue.lock().unwrap();

            if thread_queue.len() > 0 {
                // Do stuff
            }
        })
    }

    // TEST: MAKE QUEUE
    // let wq = WorkerQueue::new();
    // wq.add(query_objects.len());

    // Start handing out queries to the Workers
    // for query in query_objects {
    //     thread_pool.execute(move || {
    //         let t_wq = wq.clone();
    //         handle_query(&query);
    //         t_wq.done();
    //     });
    // }

    // TODO: We need to make the program wait (or loop?) until all the queries are finished
    // loop {
    //     // std::thread::sleep(Duration::from_millis(1));

    //     for t in thread_pool.workers {}
    // }

    // thread_pool.join().unwrap();
    // thread_pool.join();
    // wq.wait();
}

fn handle_query(query: &QueryObject) {
    println!("       Path #{} - {} : {}", query.id, 200, query.path);
    // QueryResult {
    //     id: query.id,
    //     path: query.path.clone(),
    //     status: 200,
    // }
}
