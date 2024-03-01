## Parallel computation of sum of integers stored in a file.

This programm expects a filename where integers are stored in a single line.
Then it concurently computes the sum of these integers. The programm drsigned so that the concurrent tasks share a common variable to store a sum. Every time a task updates it (adds a single integer to the current sum) it uses the mutex (together the atomic reference counter, `Arc`) to get an exclusive access.