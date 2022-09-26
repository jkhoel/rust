use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::concurrence::ThreadPool;

pub mod concurrence;

/// Simple Mutex example with no threads
pub fn simple_loop() {
    // Values to process
    let v = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let mut len = v.len();

    // Results Vector
    let r = vec![];

    println!("{:?} len: {}", v, len);

    // Create Mutexes for values shared by threads
    let mv = Mutex::new(v);
    let mr = Mutex::new(r);

    while len > 0 {
        let lv = mv.lock().unwrap();

        let new_len = len - 1;

        let i = lv[new_len] - 1;

        let mut lr = mr.lock().unwrap();
        lr.push(i);

        println!("i = {}", i);
        len = new_len
    }

    // Print the result
    let res = mr.lock().unwrap();
    println!("Res: {:?}\n\n", res);
}

/// Example showing the simple_loop() implemented with threads
pub fn concurrent_processing() {
    // Values to process
    let v = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    // Results Vector
    let r: Vec<i32> = vec![];

    // Create Mutexes for values shared by threads
    let mv = Arc::new(Mutex::new(v));
    let mr = Arc::new(Mutex::new(r));

    // Spawn threads
    let mut threads = vec![];

    // We will spawn 5 threads to handle our vector operations
    for _ in 0..5 {
        // Create a local clone of the mv vector so we can share it between threads
        let mv = Arc::clone(&mv);
        let mr = Arc::clone(&mr);

        // Spawn threads and store them in the threads vector
        let t = thread::spawn(move || {
            let mut lv = mv.lock().unwrap();

            if lv.len() > 0 {
                let c: Vec<i32> = lv.drain(0..=0).collect();
                println!("c: {:?}, lv: {:?}", c, lv);

                let mut mr = mr.lock().unwrap();
                mr.push(c[0]);
            }
        });

        threads.push(t);
    }

    // Wait until threads are finished running
    for t in threads {
        t.join().unwrap();
    }

    let res = mr.lock().unwrap();

    println!("Res: {:?}\n\n", res);
}

/// Extends the concurrent_processing() example with a queue
pub fn concurrent_processing_with_queue() {
    // Create a vector of values to process. This will essentially be our queue
    let mut v: Vec<i32> = vec![];

    for i in 0..=500_000 {
        v.push(i)
    }

    // Results Vector
    let r: Vec<i32> = vec![];

    // Create Mutexes for values shared by threads
    let mv = Arc::new(Mutex::new(v));
    let mr = Arc::new(Mutex::new(r));

    // Spawn 5 threads
    // let pool = ThreadPool::new(5);
    let pool = ThreadPool::new(50);

    // Hand out tasks to the pool until we are out of items in our queue
    loop {
        // Create clones of the Arc pointers for shared values so that the closures
        // we we pass as jobs to workers can consume them
        let mv = Arc::clone(&mv);
        let mr = Arc::clone(&mr);

        // TODO: make this exit when the last item in the queue has been processed
        if mv.lock().unwrap().len() <= 0 {
            break;
        }

        // Execute on the thread-pool!
        pool.execute(move |id| {
            // Our code to actually process the queue values goes here...
            let mut lv = mv.lock().unwrap();

            if lv.len() > 0 {
                let c: Vec<i32> = lv.drain(0..=0).collect();

                // println!("Worker #{id} :: c: {:?}, lv: {:?}", c, lv);
                // println!("Worker #{id} is working on {:?}", c[0]);

                let mut mr = mr.lock().unwrap();
                mr.push(c[0]);
            }

            // thread::sleep(Duration::from_millis(1));
        });
    }

    // let res = mr.lock().unwrap();
    // println!("Res: {:?}\n\n", res);
    println!("Done!");
}
