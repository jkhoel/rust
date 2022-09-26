use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a new asynchronous channel for the thread-pool
        let (tx, rx) = mpsc::channel();

        // We need to pass the receiver channel to multiple workers. So we wrap
        // it in `Arc` so that multiple threads can own it, and then in `Mutex`
        // to ensure only one worker can read the value at the time
        let rx = Arc::new(Mutex::new(rx));

        // Initialize a vector to hold references to our workers
        let mut workers = Vec::with_capacity(size);

        // Spawn workers (threads) and pass references to the receiver channel so that we
        // can transmit data (jobs) to them. Cloning bumps the Arc reference count so that
        // the workers can share ownership of the receiver
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        // Return a new thread-pool, supplying the sender channel so that we
        // can send to our workers

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    /// Executes a job (closure)
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(usize) + Send + 'static,
    {
        // Create a new job instance from the closure passed as f
        let job = Box::new(f);

        // Pass the job down the sending channel, and call unwrap on `send` in case sending fails
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new worker (thread)
    ///
    /// `id` is some usize we can use to id this particular worker/thread
    ///
    /// `receiver` holds a reference to the shared receiver channel used to
    /// send jobs from the main process down to the workers. In our case, ww
    /// will be sending jobs from the jobs queue
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Spawn a thread for the worker and give it the receive channel
        let thread = thread::spawn(move || loop {
            // The thread will run in an endless loop and execute the
            // closures (jobs) passed down the receiving channel as they come in
            let job = receiver.lock().unwrap().recv().unwrap();

            // println!("Worker {id} got a job; executing...");

            // Execute the closure
            job(id);
        });

        // Return a new worker
        Worker { id, thread }
    }
}

// Create a type alias for trait object Job that holds the closure (the actual job)
// we will be executing in our thread.
// See https://doc.rust-lang.org/book/ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
type Job = Box<dyn FnOnce(usize) + Send + 'static>;
