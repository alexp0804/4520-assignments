use std::char::ToLowercase;
use std::fmt::Binary;
use std::i32::MIN;
use std::thread;
use rand::distributions::{Uniform, Distribution};
use rand::rngs::ThreadRng;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};

const MIN_TEMP: i32 = -100;
const MAX_TEMP: i32 = 70;
const N_THREADS: usize = 2;
const CELLS_P_THREAD: usize = 11;
const DATA_SIZE: usize = N_THREADS * CELLS_P_THREAD;

static mut DATA: [i32; DATA_SIZE] = [0; DATA_SIZE as usize];

fn main() {
    prob2(3);
}

fn get_hourly_temperature_data() {
    let mut handles = vec![];

    unsafe {
        for chunk in DATA.chunks_mut(CELLS_P_THREAD) {

            let handle = thread::spawn(move || {

                let mut rng: ThreadRng = rand::thread_rng();
                let temps: Uniform<i32> = Uniform::from(MIN_TEMP..MAX_TEMP+1);

                // Generate temperature data for entire slice  
                for record in chunk {
                    *record = temps.sample(&mut rng);
                }
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn output_report(hour: i32) {
    unsafe {
        // Top and lowest 5 records
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();

        for i in 0..DATA_SIZE {
            max_heap.push(DATA[i]);
            min_heap.push(Reverse(DATA[i]));
        }

        // Look at each 10-minute differnce
        // TODO: this
        let mut max_10_min_delta = MIN;

        for i in 0..N_THREADS {
            let start = i * CELLS_P_THREAD;
            let end = (i + 1) * CELLS_P_THREAD;

            for j in start..end-10 {
                println!("{}", j);
                max_10_min_delta = max(max_10_min_delta, (DATA[j+10] - DATA[j]).abs());
            }
        }

        let mut highest = vec![];
        let mut lowest = vec![];

        for i in 0..5 {
            highest.push(max_heap.pop().unwrap());
            lowest.push(min_heap.pop().unwrap().0);
        }

        println!("Hour {} Report: ", hour);

        println!("Highest readings: {:?}", highest);
        println!("Lowest readings: {:?}", lowest);
        println!("Greatest temperature difference over 10 minutes: {}", max_10_min_delta);

        println!("");
    }
}

fn prob2(hours: i32) {
    for hour in 0..hours {
        unsafe {
            get_hourly_temperature_data();
            output_report(hour+1);
            println!("{:?}", DATA);
        }
    }
}