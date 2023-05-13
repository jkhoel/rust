use std::io::{BufRead, BufReader, Lines, Result as IOResult};
use std::ops::Sub;
use std::{fs::File, path::Path, sync::mpsc, thread};

pub mod concurrency;

// STRUCTS

// pub struct WorkerQueue {
//     pub size: usize,
//     pub queue_items: Vec<QueryObject>,
// }

// impl Default for WorkerQueue {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl WorkerQueue {
//     pub fn new() -> Self {
//         Self {
//             size: Mutex::new(0),
//         }
//     }

//     pub fn add(&self, num: usize) -> Self {
//         Self {
//             size: self.size + num,
//         }
//     }

//     pub fn done(&mut self) {
//         self.size = if self.size.eq(&0) {
//             0
//         } else {
//             self.size.sub(1)
//         }
//     }

//     pub fn size(&self) -> usize {
//         self.size.clone()
//     }

//     pub fn wait(&self) {
//         let queue = self.size;

//         if queue.eq(&0) {
//             return;
//         }

//         while queue > 0 {
//             // Do nothing?
//         }
//     }
// }

/// Represents a path we are trying to enumerate
pub struct QueryObject {
    pub id: usize,
    /// A path to enumerate
    pub path: String,
    /// The HTTP status code returned for that path
    pub status: u8,
}

impl QueryObject {
    // Instanciates one signal QueryObjects
    pub fn new(path: &String, id: usize) -> Result<QueryObject, &'static str> {
        if !path.is_empty() {
            return Err("Unable to read path from file");
        }

        Ok(QueryObject {
            id,
            path: path.clone(),
            status: 0,
        })
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status
    }
}

/// Creates a thread-pool for the Workers to live in
///
/// Source: [Ch20-02 Mulithreading in the Rust Book](https://doc.rust-lang.org/book/ch20-02-multithreaded.html)
// pub struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Job>,
//     queue: usize,
// }

// impl ThreadPool {
//     /// Creates a new ThreadPool
//     ///
//     /// The size is the number of threads to spawn in the pool.
//     ///
//     /// # Panics
//     ///
//     /// The `new` function will panic if the size is zero
//     pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
//         if size < 1 {
//             return Err("size must be above 0");
//         }

//         // Create an async channel for sending/recieving data
//         let (sender, receiver) = mpsc::channel();

//         // Use Arc to allow multiple workers own the reciever, and Mutex to ensure only
//         // one worker gets a job from the reciever at a time.
//         let receiver = Arc::new(Mutex::new(receiver));

//         // Create workers for the thread-pool, and hook them up to the receiver channel
//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             println!("    > Spawning Worker for Thread #{}", id + 1);
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         Ok(ThreadPool {
//             workers,
//             sender,
//             queue: size,
//         })
//     }

//     pub fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);

//         // Send the job down the channel
//         self.sender.send(job).expect();
//     }

//     pub fn join(&self) {
//         loop {
//             if self.queue < 1 {
//                 break;
//             }

//             println!("Remaining in queue: {}", self.queue)
//         }
//     }

//     // pub fn join(&self) {
//     //     loop {
//     //         // std::thread::sleep(Duration::from_millis(1));

//     //         for worker in self.workers.iter() {
//     //             worker.thread.join().unwrap()
//     //         }
//     //     }
//     // }
// }

// // Type alias for a Box that holds each closure
// type Job = Box<dyn FnOnce() + Send + 'static>;

// // Handles running some code on one of the threads in the pool
// struct Worker {
//     id: usize,
//     thread: thread::JoinHandle<()>,
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread = thread::spawn(move || loop {
//             let job = receiver.lock().unwrap().recv().unwrap();

//             println!("    ...Worker {id} got a job; executing");
//             job()
//         });

//         Worker { id, thread }
//     }
// }

// FUNCTIONS

/// Converts a positive number as String into an usize (u8, u16 etc..)
///
/// `val` is the string we are converting to some integer
///
/// # Panics
///
/// The function will panic if `val` is less than 0
pub fn string_to_usize(val: &str) -> Result<usize, &str> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(val),
    }
}

///
/// Reads a file and returns a Lines-iterator
///
pub fn read_lines_from_file<P>(filename: P) -> IOResult<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

///
/// Parses a wordlist into a Vector of QueryObjects
///
pub fn digest_wordlist(path: &String) -> Result<Vec<QueryObject>, &'static str> {
    if path.is_empty() {
        return Err("Missing list argument");
    }

    // Read in paths from the wordlist
    // let paths = read_lines_from_file(&path).into_iter(); // .expect("Unable to read lines from wordlist");
    let paths = read_lines_from_file(&path).expect("Unable to read lines from wordlist");

    // Define a vector to hold our paths as QueryObjects, and populate it
    let mut vec: Vec<QueryObject> = vec![];

    for (index, path) in paths.enumerate() {
        let p = path.unwrap();

        if !p.starts_with('#') {
            vec.push(QueryObject {
                id: index,
                path: p,
                status: 0,
            });
        }
    }

    Ok(vec)
}
