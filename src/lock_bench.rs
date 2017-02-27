use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{Ordering, AtomicUsize};

const N: i32 = 10000;

pub fn with_mutex(num_reader: i32, num_writer: i32) {
    let mut ths = Vec::new();
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..num_writer {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                let mut counter_ref = counter.lock().unwrap();
                *counter_ref += 1;
            }
        }));
    }
    
    for _ in 0..num_reader {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                let _ = counter.lock().unwrap();
            }
        }));
    }
    
    for th in ths {
        th.join().unwrap();
    }
}

pub fn with_rwlock(num_reader: i32, num_writer: i32) {
    let mut ths = Vec::new();
    let counter = Arc::new(RwLock::new(0));

    for _ in 0..num_writer {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                let mut counter_ref = counter.write().unwrap();
                *counter_ref += 1;
            }
        }));
    }
    
    for _ in 0..num_reader {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                let _ = counter.read().unwrap();
            }
        }));
    }
    
    for th in ths {
        th.join().unwrap();
    }
}

pub fn with_atomic(num_reader: i32, num_writer: i32) {
    let mut ths = Vec::new();
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..num_writer {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    
    for _ in 0..num_reader {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                let _ = counter.load(Ordering::SeqCst);
            }
        }));
    }
    
    for th in ths {
        th.join().unwrap();
    }
}

