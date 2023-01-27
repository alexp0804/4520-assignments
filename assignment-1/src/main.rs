use bitvec::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

// The last prime is at most sqrt(MAX_NUM). The size of each block is MAX_NUM/N_THREADS.
// With 8 threads, this means that for any value of MAX_NUM >= 64, the last prime we need will always be in the first block.
// If we have less than 64, it's probably best to just llimit the number of threads. I chose to limit it to 1 thread, because additional threads
// hardly affects performance for such a low amount of work to be done.
const N_THREADS: usize = 8;
const MAX_NUM: usize = 10_usize.pow(8);
// const MAX_NUM: usize = 10;
const TOP_N: usize = 10;

// Computes the primes from 3 to sqrt(n) so all threads can reference it while sieving, with no overhead from communication
fn compute_pre_sieve(n: i32) -> Vec<usize> {
    let mut pre_sieve = bitvec![1; n as usize];

    // It's just easier to manually set 0 and 1 to false
    pre_sieve.set(0 as usize, false);
    pre_sieve.set(1 as usize, false);

    // Sift
    let mut i = 1;
    while i * i <= n {
        // Found a prime
        if pre_sieve[i as usize] {
            // Remove multiples of the prime
            let mut j = i * i;
            while j < n {
                pre_sieve.set(j as usize, false);
                j += i;
            }
        }

        i += 2;
    }

    // Put the primes into a vector and return it
    let mut ret: Vec<usize> = vec![];
    for i in 0..n {
        if pre_sieve[i as usize] {
            ret.push(i as usize);
        }
    }

    ret
}

fn main() {
    let start = Instant::now();
    let pre_sieve = compute_pre_sieve(((MAX_NUM as f32).sqrt()) as i32);
    let prime_info = Arc::new(Mutex::new((0, 0)));
    let prime_list = Arc::new(Mutex::new(vec![0; TOP_N]));
    let mut handles = vec![];

    let n_threads = if MAX_NUM > 64 { N_THREADS } else { 1 };
    let block_size = MAX_NUM / n_threads;

    for thread in 0..n_threads {
        // Every thread gets a copy of the primes from 2..sqrt(n)
        let primes = pre_sieve.clone();
        let prime_info = Arc::clone(&prime_info);
        let prime_list = Arc::clone(&prime_list);

        let handle = thread::spawn(move || {
            // We give each thread a portion of the numbers from 1..MAX_NUM
            let lo = thread * block_size;
            let hi = (thread + 1) * block_size
                + (thread == n_threads - 1) as usize * (MAX_NUM % n_threads);
            let mut sieve_block = bitvec![1; hi - lo];

            // 0 and 1 are not prime
            if thread == 0 {
                sieve_block.set(0, false);
                sieve_block.set(1, false);
            }

            for prime in primes {
                // Fist occurence of a multiple of this prime in this range that is not the prime itself
                let mut i = if lo % prime == 0 {
                    lo
                } else {
                    lo + prime - lo % prime
                };
                if thread == 0 {
                    i += 2 * prime
                };

                while i < hi {
                    sieve_block.set(i - lo, false);
                    i += prime;
                }
            }

            // Count the number of primes found and sum their values
            let mut sum: usize = 0;
            let mut count: usize = 0;
            let mut top_n: Vec<i32> = vec![];

            // Iterate backwards so we can add the top values immediately
            for (i, bit) in sieve_block.iter().enumerate().rev() {
                if *bit {
                    if top_n.len() < TOP_N {
                        top_n.push((i + lo) as i32);
                    }
                    sum += i + lo;
                    count += 1;
                }
            }
            // Only the last thread will have the TOP_N primes because it had the last chunk of the number line
            if thread == n_threads - 1 {
                let mut list = prime_list.lock().unwrap();
                top_n.sort();
                *list = top_n;
                drop(list);
            }

            let mut my_info = prime_info.lock().unwrap();
            my_info.0 += count;
            my_info.1 += sum;

            // Mic drop
            drop(my_info);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = *prime_info.lock().unwrap();
    let prime_list = prime_list.lock().unwrap().clone();
    let duration = start.elapsed();
    println!("{:.2?} {} {}", duration, results.0, results.1);
    for prime in prime_list {
        if prime != 0 {
            print!("{} ", prime);
        }
    }
    println!();
}
