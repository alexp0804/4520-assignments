# Design
My design involves using eight threads to simultaneously check the range of numbers from $2$ to $10^8$ for their primality.

The function for checking if a number $x$ is prime uses an iterative approach, checking to see if each number from $2$ up until $\lfloor \sqrt x \rfloor$ divides $x$. If any number in that range divides $x$, the number is not prime. If all numbers in that range do not divide $x$, then $x$ is prime.

The process involves each thread utilizing a shared counter to communicate what number needs to be checked next. Once a thread sees the next number, it increments the counter.

The thread then computes that numbers primality. If it's prime, the thread adds the value to a local sum, increments a local counter, and adds it to a list of the largest primes it's seen.

Once a thread is finished, it's information is added to the information in the main thread, where it gets displayed when all threads are done.

# Efficiency
This approach is efficient because each of the threads are able to work independently of each other easily, as no thread relies on another. The only time a thread has to wait on another is when getting the next number from the counter and when merging heaps at the end of it's computations. The counter is held for a minimal amount of time, so for most of the computation time, the threads do not wait on each other.


# Evaluation
On my system, computing the primes up until $10^8$ with $8$ threads took an average of $8.07$ seconds. I ran the computation $100$ times to compute this average. I also ran the code with only a single thread, which took approximately $55.31$ seconds on average.

This shows a runtime decrease of about $85\%$.

# Compiling and Running
Navigate to the ``assignment-1`` directory and run with `cargo run --release`.

[Guide to install Cargo or Rust, if it is not installed.](https://doc.rust-lang.org/book/ch01-01-installation.html#installation)