
// Alexander Peterson
// COP 4520 - Assignment 1

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Constants
    let max_candidate = 10_i64.pow(8);
    let n_threads = 8;

    // Thread variables
    let next_candidate = Arc::new(Mutex::new(1));
    let prime_heap = Arc::new(Mutex::new(BinaryHeap::<Reverse<i64>>::new()));
    // # of primes, sum(primes)
    let prime_info = Arc::new(Mutex::new((0, 0)));
    
    let mut handles = vec![];
    let start = Instant::now();

    for _ in 0..n_threads {
        let next_candidate = Arc::clone(&next_candidate);
        let prime_heap = Arc::clone(&prime_heap);
        let prime_info = Arc::clone(&prime_info);

        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            let mut local_count = 0;
            let mut local_heap = BinaryHeap::<Reverse<i64>>::new();

            loop {
                // Acquire candidate lock, increment, save the value for this thread, then drop it
                let mut candidate = next_candidate.lock().unwrap();
                let my_candidate = *candidate;

                // No need to check even numbers (other than 2)
                *candidate += 1 + (*candidate > 2) as i64;
                drop(candidate);

                if my_candidate > max_candidate {
                    break;
                }

                if is_prime(my_candidate) {
                    local_heap.push(Reverse(my_candidate));

                    // If storing too many primes, trim it down
                    while local_heap.len() > 10 {
                        local_heap.pop();
                    }

                    // Update local sum and count
                    local_sum += my_candidate;
                    local_count += 1;
                }
            }

            // Once done looping, update the main sum, main prime count and the main heap
            let mut my_info = prime_info.lock().unwrap();
            my_info.0 += local_count;
            my_info.1 += local_sum;
            drop(my_info);

            let mut my_heap = prime_heap.lock().unwrap();
            my_heap.append(&mut local_heap);
            drop(my_heap);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();

    // Output execution time, number of primes, and sum of all primes found, and top 10 primes.
    let results = *prime_info.lock().unwrap();
    println!("{:.2?} {} {}", duration, results.0, results.1);

    let mut max_primes = prime_heap.lock().unwrap().clone();
    while max_primes.len() > 10 {
        max_primes.pop();
    }
    for prime in max_primes.into_sorted_vec().iter().rev() {
        print!("{} ", prime.0);
    }
    println!();
}

// Iterative approach for checking a number's primality
// O(sqrt(n)) time, O(1) space
fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    let mut i: i64 = 2;
    while (i*i) <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}