use threads_shared_data::{concurrent_processing, concurrent_processing_with_queue, simple_loop};

/**
 * Source: https://doc.rust-lang.org/book/ch16-03-shared-state.html
 *
 * Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one
 * thread to access some data at any given time. To access the data in a mutex,
 * a thread must first signal that it wants access by asking to acquire the mutex’s lock.
 * The lock is a data structure that is part of the mutex that keeps track of who currently
 * has exclusive access to the data. Therefore, the mutex is described as guarding the data
 * it holds via the locking system.
 *
 * Mutexes have a reputation for being difficult to use because you have to remember two rules:
 * - You must attempt to acquire the lock before using the data.
 * - When you’re done with the data that the mutex guards, you must unlock the data so other
 *   threads can acquire the lock.
 */

fn main() {
    // This function is a very basic example to show locking of shared values
    simple_loop();

    // Same as simple_loop(), but this time using multithreading to process the vector.
    // Notice however that this particular example requires the same amount of workers as
    // we have values to process
    concurrent_processing();

    // This function solves the 1:1 requrement between workers and values by implementing a queue
    concurrent_processing_with_queue();
}
