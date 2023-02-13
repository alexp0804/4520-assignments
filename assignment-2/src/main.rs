use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const N: usize = 50;

fn prob1() {
    let cupcake = Arc::new(Mutex::new(true));
    let done_signal = Arc::new(Mutex::new(false));

    // Used only for validation purposes
    let visited = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..N {
        let mut requested: usize = 0;
        let mut ate = false;

        let cupcake = Arc::clone(&cupcake);
        let visited = Arc::clone(&visited);
        let done_signal = Arc::clone(&done_signal);

        let handle = thread::spawn(move || {
            loop {
                // Check if the leader said we can stop
                let done = *done_signal.lock().unwrap();
                if done {
                    break;
                }
                drop(done);

                let mut is_cupcake = cupcake.lock().unwrap();

                // The leader
                if i == 0 {
                    // Leader requests a cupcake if non is present.
                    if !*is_cupcake {
                        requested += 1;
                        *is_cupcake = true;
                    }
                    // Eat a cupcake, if they haven't.
                    if !ate {
                        ate = true;
                        requested += 1;

                        // Mark that this guest visited for validation
                        let mut my_visited = visited.lock().unwrap();
                        *my_visited += 1;
                    }

                    drop(is_cupcake);

                    // Send the signal to the Minotaur stating that everyone has ate a cupcake.
                    if requested >= N {
                        let mut we_done = done_signal.lock().unwrap();
                        *we_done = true;
                        break;
                    }

                // Non-leader guest
                } else {
                    // This guest already ate. Leave the maze, regardless if there's a cupcake or not.
                    if ate {
                        continue;
                    }

                    if *is_cupcake {
                        // Mark that this guest visited for validation
                        let mut my_visited = visited.lock().unwrap();
                        *my_visited += 1;

                        // Eat cupcake
                        *is_cupcake = false;
                        ate = true;
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut visited_info = visited.lock().unwrap().clone();
    let all_visited = visited_info == N;

    if all_visited {
        println!("All {} guests indeed ate a cupcake!", N);
    } else {
        println!("Not all {} guests ate a cupcake...", N);
    }
}

fn prob2() {
    let mut handles = vec![];
    let next_in_line = Arc::new(Mutex::new(0));

    // Only used for validation purposes
    let visited = Arc::new(Mutex::new(0));

    for i in 0..N {
        let visited = Arc::clone(&visited);
        let next_in_line = Arc::clone(&next_in_line);

        let handle = thread::spawn(move || {
            loop {
                // Get next in line, check if its this guest
                let mut next = next_in_line.lock().unwrap();

                // Visiting the room...
                if *next == i {
                    let mut visit = visited.lock().unwrap();
                    *visit += 1;
                    *next += 1;
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut visited_info = visited.lock().unwrap().clone();
    let all_visited = visited_info == N;

    if all_visited {
        println!("All {} guests in line visited the room.", N);
    } else {
        println!("Not all {} guests in line visited the room...", N);
    }
}

fn main() {
    // Running problem 1...
    let mut start = Instant::now();
    prob1();
    let mut duration = start.elapsed();
    println!("Problem #1 took: {:.2?}\n", duration);

    // Running problem 2...
    start = Instant::now();
    prob2();
    duration = start.elapsed();
    println!("Problem #2 took: {:.2?}", duration);
}
