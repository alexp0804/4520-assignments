# Design

My design splits the range from $[2, {10}^8]$ into approximately evenly sized chunks, one for each thread. Each thread gets a precomputed list of the first $[2, \sqrt {{10}^8}]$ primes, so they can all work independently.

This involves computing the first $[2, \sqrt {{10}^8}]$ primes sequentially. However, since the range is small, this can be done efficiently with just one thread.

# Efficiency
Because each thread works entirely independent of the others, we achieve maximum speedup from parallelism. The entire task of sifting composites from the range of $[2, \sqrt {10}^8]$ is parallelized.

Each thread gets approximately the same amount of work to do. The only exception is the last thread, which might have a slightly larger range if `N_THREADS` does not divide `MAX_NUM`. The amount of added numbers this range needs to check is at most `N_THREADS`, so it is hardly a concern.

I avoid extra memory by using a `BitVec` instead of a standard `Vec<bool>`. A `bool` is usually $8$ bits, but we can get away with using just $1$ in our sieve, so this uses an eigth of the space as a `Vec<bool>` implementation.

# Evaluation
On my system running an i5-13600k, computing the primes up until ${10}^8$ with $8$ threads took an average of $83.34$ milliseconds, over the course of $1000$ runs.

I also tested the program to find primes numbers in a different range. When I set `MAX_NUM` to be $10$, it found the primes $2$, $3$, $5$, $7$ and their sum $17$, which is correct. I also tried setting `MAX_NUm` to $100$, and it found the top primes to be $53$, $59$, $61$, $67$, $71$, $73$, $79$, $83$, $89$, $97$, with a sum of $1060$, which is correct, again.

# Compiling and Running
Navigate to the ``assignment-1`` directory and run with `cargo run --release`.
The output file should appear in the `assignment-1` directory, named `primes.txt`.

You must use `cargo` to build this, as it relies on a dependency for the `BitVec`.

[Guide to install Cargo or Rust, if it is not installed.](https://doc.rust-lang.org/book/ch01-01-installation.html#installation)
