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

pub fn iter(num_reader: usize, num_writer: usize) {
    println!("==== num_reader: {}, num_writer: {} ====", num_reader, num_writer);

    let mut bm_mutex = Vec::new();
    let mut bm_rwlock = Vec::new();
    let mut bm_parking_rwlock = Vec::new();
    let mut bm_atomic_seq_cst = Vec::new();
    let mut bm_atomic_relaxed = Vec::new();
    let mut bm_unsynchronized = Vec::new();

    for i in 0..N {
        if i > 0 {
            bm_mutex.push(run(&|| with_mutex(num_reader, num_writer)));
            bm_rwlock.push(run(&|| with_rwlock(num_reader, num_writer)));
            bm_parking_rwlock.push(run(&|| with_parking_rwlock(num_reader, num_writer)));
            bm_atomic_seq_cst.push(run(&|| with_atomic_seq_cst(num_reader, num_writer)));
            bm_atomic_relaxed.push(run(&|| with_atomic_relaxed(num_reader, num_writer)));
            bm_unsynchronized.push(run(&|| with_unsynchronized(num_reader, num_writer)));
        }
    }

    print_result("mutex", &bm_mutex);
    print_result("rwlock", &bm_rwlock);
    print_result("prwlock", &bm_rwlock);
    print_result("seq_cst", &bm_atomic_seq_cst);
    print_result("relaxed", &bm_atomic_relaxed);
    print_result("unsync", &bm_unsynchronized);
    println!("");
}

pub fn main() {
    let total = 20usize;
    for num_writer in [0, 1, 2, 4, 8, 12, 16, 20].iter() {
        iter(total - num_writer, *num_writer);
    }
}
