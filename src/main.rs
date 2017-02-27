extern crate statrs;
extern crate time;

extern crate lock_bench;

use statrs::statistics::Statistics;
use time::{PreciseTime};

use lock_bench::*;

const N : i32 = 8;

fn run(f: &Fn()) -> f64 {
    let start = PreciseTime::now();
    f();
    let end = PreciseTime::now();
    let d = start.to(end);
    d.num_milliseconds() as f64
}

fn print_result(label: &str, result: &Vec<f64>) {
    println!("{:8} =>   mean: {:8.2}, std_dev: {:8.2}",
             label, result.clone().mean(), result.clone().std_dev());
}

pub fn iter(num_reader: i32, num_writer: i32) {
    println!("==== num_reader: {}, num_writer: {} ====", num_reader, num_writer);

    let mut bm_mutex = Vec::new();
    let mut bm_rwlock = Vec::new();
    let mut bm_atomic = Vec::new();

    for i in 0..N {
        if i > 0 {
            bm_mutex.push(run(&|| with_mutex(num_reader, num_writer)));
            bm_rwlock.push(run(&|| with_rwlock(num_reader, num_writer)));
            bm_atomic.push(run(&|| with_atomic(num_reader, num_writer)));
        }
    }

    print_result("mutex", &bm_mutex);
    print_result("rwlock", &bm_rwlock);
    print_result("atomic", &bm_atomic);
    println!("");
}

pub fn main() {
    let total = 20;
    for num_writer in [0, 1, 2, 4, 8, 12, 16, 20].iter() {
        iter(total - num_writer, *num_writer);
    }
}

