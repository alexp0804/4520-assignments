use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use rand::Rng;

const N: usize = 100;
const CHANCE_TO_ENTER: f32 = 0.7;
const CHANCE_TO_REENTER: f32 = 0.2;

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

    let visited_info = visited.lock().unwrap().clone();
    let all_visited = visited_info == N;

    if all_visited {
    //     println!("All {} guests indeed ate a cupcake!", N);
    } else {
    //     println!("Not all {} guests ate a cupcake...", N);
    }
}

fn prob2() {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let next_thread = Arc::new(Mutex::new(0));
    let done_signal = Arc::new(Mutex::new(false));
    
    let mut handles = vec![];
    let mut random = rand::thread_rng();

    for i in 0..N {
        // Does this guest want to get in line?
        if random.gen::<f32>() < CHANCE_TO_ENTER {
            let queue_ref = Arc::clone(&queue);
            let next_thread_ref = Arc::clone(&next_thread);
            let done_ref = Arc::clone(&done_signal);

            let handle = thread::spawn(move || {
                loop {
                    // No more guests in the queue, stop running
                    if *done_ref.lock().unwrap() {
                        break;
                    }

                    // Are we next in line? If not, try again later
                    let mut next_thread = next_thread_ref.lock().unwrap();
                    if i != *next_thread {
                        continue;
                    }

                    let mut t_queue = queue_ref.lock().unwrap();

                    // Does this guest want to get in line again?
                    if rand::thread_rng().gen::<f32>() < CHANCE_TO_REENTER {
                        t_queue.push_back(i);
                    }

                    // Try to call the next guest
                    match t_queue.pop_front() {
                        // No guests left, end the program
                        None => {
                            *done_ref.lock().unwrap() = true;
                        },
                        // Tell the next guest to enter the room.
                        Some(index) => {
                            *next_thread = index;
                        }

                    }
                }
            });

            handles.push(handle);
            (*queue.lock().unwrap()).push_back(i);
        }
    }

    // Let the first guest in, they'll take care of calling the next guest, and so on.
    match queue.lock().unwrap().pop_front() {
        // Not very popular today...
        None => return,
        Some(index) => {
            *next_thread.lock().unwrap() = index;
        }
    };

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // Running problem 1...
    let mut start = Instant::now();
    prob1();
    let mut duration = start.elapsed();
    println!("Problem #1 took: {:.2?}", duration);

    // Running problem 2...
    start = Instant::now();
    prob2();
    duration = start.elapsed();
    println!("Problem #2 took: {:.2?}", duration);
}
