use bitvec::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/*
    Computes Sieve of Eratosthanes by giving each thread a subset of (0..MAX_NUM), where all subsets are disjoint.
To avoid communication between threads, we precompute all primes needed to sieve up to MAX_NUM before going parallel.
This drastically improves speed, at the expense of needing to store the primes from [2..sqrt(MAX_NUM)].

For MAX_NUM = 100_000_000, that only turns out to be about 10_000 numbers, which is minimal enough.

If MAX_NUM is small enough, we use only one thread. The overhead of spawning and maintaining threads for such a small
range to compute is not worth it.

We want to ensure the last precomputed prime is within the first block, so that we can use a simple parallel algorithm.
We know:
    LastPrePrime ~= sqrt(MAX_NUM),
     and
    BlockSize = MAX_NUM / N_THREADS
     therefore
    LastPrePrime <= BlockSize   ==>   sqrt(MAX_NUM) <= MAX_NUM / N_THREADS
     so
    N_THREADS >= MAX_NUM / sqrt(MAX_NUM)

    With N_THREADS = 8, we can find
        MAX_NUM <= 64

    So for any value MAX_NUM <= 64, we switch to single-threading.
*/

const N_THREADS: usize = 8;
const MAX_NUM: usize = 10_usize.pow(8);
const TOP_N: usize = 10;

fn compute_pre_sieve(n: usize) -> Vec<usize> {
    let mut pre_sieve = bitvec![1; n as usize];

    // 0 and 1 are not prime.
    pre_sieve.set(0 as usize, false);
    pre_sieve.set(1 as usize, false);

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

        // Skip even numbers
        i += 2;
    }

    // Put the primes into a vector and return it
    let mut ret: Vec<usize> = vec![];
    for i in 0..n {
        if pre_sieve[i] {
            ret.push(i);
        }
    }

    ret
}

// Gets the range, lo to hi, for this thread to sieve across
fn compute_range(thread_idx: usize, block_size: usize, n_threads: usize) -> (usize, usize) {
    (
        thread_idx * block_size,
        (thread_idx + 1) * block_size
            + (thread_idx == n_threads - 1) as usize * (MAX_NUM % n_threads),
    )
}

// Gets the first occurence of a multiple of this prime in this range, that is not the prime itself
fn get_first_occurence(prime: usize, lo: usize, block_idx: usize) -> usize {
    let mut first = if lo % prime == 0 {
        lo
    } else {
        lo + prime - lo % prime
    };

    if block_idx == 0 {
        first += 2 * prime;
    }

    first
}

fn main() {
    let start = Instant::now();
    let pre_sieve = compute_pre_sieve(((MAX_NUM as f32).sqrt()) as usize);

    // Sum and count of primes, top N primes
    let prime_info = Arc::new(Mutex::new((0, 0)));
    let prime_list = Arc::new(Mutex::new(vec![0; TOP_N]));

    let n_threads: usize = if MAX_NUM > 64 { N_THREADS } else { 1 };
    let block_size: usize = MAX_NUM / n_threads;

    let mut handles = vec![];
    for t in 0..n_threads {
        // Every thread gets a copy of the primes from [2..sqrt(n)]
        let primes = pre_sieve.clone();
        let prime_info = Arc::clone(&prime_info);
        let prime_list = Arc::clone(&prime_list);

        let handle = thread::spawn(move || {
            // We give each thread a portion of the numbers from 1..MAX_NUM
            let (lo, hi) = compute_range(t, block_size, n_threads);
            let mut sieve_block = bitvec![1; (hi - lo) as usize];

            // 0 and 1 are not prime
            if t == 0 {
                sieve_block.set(0 as usize, false);
                sieve_block.set(1 as usize, false);
            }

            // Remove these composites!
            for prime in primes {
                let mut i = get_first_occurence(prime, lo, t);

                while i < hi {
                    sieve_block.set((i - lo) as usize, false);
                    i += prime;
                }
            }

            let mut sum: usize = 0;
            let mut count: usize = 0;
            let mut top_n: Vec<usize> = vec![];

            // Iterate backwards so we can add the top values immediately
            for (i, bit) in sieve_block.iter().enumerate().rev() {
                if *bit {
                    let p = i + lo;
                    if top_n.len() < TOP_N {
                        top_n.push(p);
                    }
                    sum += p;
                    count += 1;
                }
            }

            let mut list = prime_list.lock().unwrap();
            list.append(&mut top_n);
            list.sort();
            if list.len() > TOP_N {
                let start = list.len() - TOP_N ;
                list.drain(0..start);
            }
            drop(list);

            let mut my_info = prime_info.lock().unwrap();
            my_info.0 += count;
            my_info.1 += sum;
            drop(my_info);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final results!
    let results = *prime_info.lock().unwrap();
    let prime_list = prime_list.lock().unwrap().clone();
    let duration = start.elapsed();

    let mut file = File::create("primes.txt").unwrap();
    writeln!(&mut file, "{:.2?} {} {}", duration, results.0, results.1).unwrap();
    for prime in prime_list {
        if prime != 0 {
            write!(&mut file, "{} ", prime).unwrap();
        }
    }
    writeln!(&mut file, "").unwrap();
}
