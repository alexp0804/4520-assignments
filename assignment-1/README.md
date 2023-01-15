# Design
My design involves using eight threads to simultaneously check the range of numbers from $2$ to $10^8$ (skipping most even numbers) for their primality using a shared counter variable to indicate which number needs to be checked next.

The function for checking if a number $x$ is prime uses an iterative approach, checking to see if each number from $2$ up until $\lfloor \sqrt x \rfloor$ (again, skipping most even numbers) divides $x$. If any number in that range divides $x$, the number is not prime. If all numbers in that range do not divide $x$, then $x$ is prime.

Once the counter variable exceeds the last candidate number, each thread brings what it found to the main thread, where the information is then outputted.

# Efficiency
This approach is efficient because each of the threads are able to work independently of each other, as no thread relies on another to check primality. The only time a thread has to wait on another is when getting the next candidate from the counter or if merging it's information after all computation is done. However, since the counter is held for a minimal amount of time, the threads do not wait on each other for a majority of computation time, allowing for efficient computation.

Each thread gets approximately the same amount of work to do. All numbers close to a value of the counter take approximately the same amount of time to check if a candidate is prime. Since the threads use the counter as the next candidate, at any given time the threads will have an approximately equal amount of work.

I also decreased the number of candidates to check by half by ignoring every even number other than two. I used the fact that the only even number I would be checking for primality is 2 to optimize my `is_prime()` function by skipping even factors. This significantly cuts down computation time as the candidates get larger.

# Evaluation
On my system running an i5-13600k, computing the primes up until $10^8$ with $8$ threads took an average of $3.60$ seconds. I also ran the code with only a single thread, which took approximately $28.02$ seconds on average.

This shows a runtime decrease of about $85$%.

# Compiling and Running
Navigate to the ``assignment-1`` directory and run with `cargo run --release`.

Alternatively, in the ``assignment-1/src`` directory, compile with ``rustc -C opt-level=3 main.rs`` and run the executable file it creates with ``./main``.

[Guide to install Cargo or Rust, if it is not installed.](https://doc.rust-lang.org/book/ch01-01-installation.html#installation)
