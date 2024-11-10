A `channel` is a one-way conduit for sending values from one thread to another.
In other words, it’s a __thread-safe queue__.

With channels, threads can communicate by passing values to one another.
It’s a very simple way for threads to work together without using locking or shared memory.

Rust channels are faster than Unix pipes. Sending a value moves it rather than copying it,
and moves are fast even when you’re moving data structures that contain many megabytes of data.

This project is an example of how to use `tokio` channels to create a pipeline of `tokio` tasks.
The pipeline works as following
```
[ Read file ] => [ Split into words ] =>  [ Split into letters and store their frequency in a HashMap ]
```

