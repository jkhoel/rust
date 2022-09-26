# Multithreading

This project demos handing out a queue of work to threads via workers.

The threads will share the same data, meaning they share both the queue and the result vectors.

For an introduction to Arc and Mutex, see [Ch.16.3 of the Rust Book](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
