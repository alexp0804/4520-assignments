
// Alexander Peterson
// COP 4520 - Assignment 1

use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_CANDIDATE: i32 = 10_i32.pow(8);
const N_THREADS: i32 = 8;
const N_PRIMES: usize = 10;

fn main() {
    // Thread variables
    let next_candidate = Arc::new(Mutex::new(1));
    let prime_list = Arc::new(Mutex::new(vec![0; N_PRIMES]));
    // (number of primes, sum of primes)
    let prime_info = Arc::new(Mutex::new((0, 0)));

    let mut handles = vec![];
    let start = Instant::now();

    for _ in 0..N_THREADS {
        let next_candidate = Arc::clone(&next_candidate);
        let prime_info = Arc::clone(&prime_info);
        let prime_list = Arc::clone(&prime_list);

        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            let mut local_count = 0;
            let mut local_list = vec![0; N_PRIMES];
            let mut local_idx = 0;

            loop {
                // Acquire candidate lock, increment, save the value for this thread, then drop it
                let mut candidate = next_candidate.lock().unwrap();
                let my_candidate = *candidate;

                // No need to check even numbers (other than 2)
                *candidate += 1 + (*candidate > 2) as i32;
                drop(candidate);

                if my_candidate > MAX_CANDIDATE {
                    break;
                }

                if is_prime(my_candidate) {
                    local_list[local_idx] = my_candidate;
                    local_idx = (local_idx + 1) % N_PRIMES;

                    // Update local sum and count
                    local_sum += my_candidate as i64;
                    local_count += 1;
                }
            }

            // Once done looping, update the main sum, main prime count and the main list
            let mut my_info = prime_info.lock().unwrap();
            my_info.0 += local_count;
            my_info.1 += local_sum as i64;
            drop(my_info);

            let mut my_list = prime_list.lock().unwrap();
            my_list.append(&mut local_list);
            my_list.sort();
            my_list.drain(0..N_PRIMES);
            drop(my_list);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();

    // Output execution time, number of primes, and sum of all primes found, and top 10 primes
    let results = *prime_info.lock().unwrap();
    println!("{:.2?} {} {}", duration, results.0, results.1);

    let prime_list = prime_list.lock().unwrap().clone();
    for prime in prime_list {
        if prime != 0 {
            print!("{} ", prime);
        }
    }

    println!();
}

// Iterative approach for checking a number's primality
// O(sqrt(n)) time, O(1) space
// Uses the 6k +- 1 trick detailed in this Wikipedia article:
// https://en.wikipedia.org/wiki/Primality_test#Simple_methods 
fn is_prime(n: i32) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: i32 = 5;
    while (i*i) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}
