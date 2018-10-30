use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicUsize};

use std::sync::RwLock as StdRwLock;
extern crate parking_lot;
use parking_lot::RwLock as ParkingRwLock;

const N: usize = 10000;

pub fn with<T, G>(num_reader: usize, num_writer: usize, gen: G, write: fn(&Arc<T>), read: fn(&Arc<T>))
    where T: Sync + Send + 'static, G: Fn() -> T {
    let mut ths = Vec::new();
    let counter = Arc::new(gen());

    for _ in 0..num_writer {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                write(&counter)
            }
        }));
    }

    for _ in 0..num_reader {
        let counter = counter.clone();
        ths.push(thread::spawn(move || {
            for _ in 0..N {
                read(&counter)
            }
        }));
    }

    for th in ths {
        th.join().unwrap();
    }
}

pub fn with_mutex(num_reader: usize, num_writer: usize) {
    fn write(counter: &Arc<Mutex<usize>>) {
         let mut counter_ref = counter.lock().unwrap();
         *counter_ref += 1;
    }

    fn read(counter: &Arc<Mutex<usize>>) {
         let _ = counter.lock().unwrap();
    }

    with(num_reader, num_writer, || Mutex::new(0), write, read);
}

pub fn with_rwlock(num_reader: usize, num_writer: usize) {
    fn write(counter: &Arc<StdRwLock<usize>>) {
        let mut counter_ref = counter.write().unwrap();
        *counter_ref += 1;
    }

    fn read(counter: &Arc<StdRwLock<usize>>) {
        let _ = counter.read().unwrap();
    }

    with(num_reader, num_writer, || StdRwLock::new(0), write, read);
}

pub fn with_parking_rwlock(num_reader: usize, num_writer: usize) {
    ;
    fn write(counter: &Arc<ParkingRwLock<usize>>) {
        let mut counter_ref = counter.write();
        *counter_ref += 1;
    }

    fn read(counter: &Arc<ParkingRwLock<usize>>) {
        let _ = counter.read();
    }

    with(num_reader, num_writer, || ParkingRwLock::new(0), write, read);
}

pub fn with_atomic_seq_cst(num_reader: usize, num_writer: usize) {
    fn write(counter: &Arc<AtomicUsize>) {
        counter.fetch_add(1, Ordering::SeqCst);
    }

    fn read(counter: &Arc<AtomicUsize>) {
        let _ = counter.load(Ordering::SeqCst);
    }

    with(num_reader, num_writer, || AtomicUsize::new(0), write, read);
}

pub fn with_atomic_relaxed(num_reader: usize, num_writer: usize) {
    fn write(counter: &Arc<AtomicUsize>) {
        counter.fetch_add(1, Ordering::Relaxed);
    }

    fn read(counter: &Arc<AtomicUsize>) {
        let _ = counter.load(Ordering::Relaxed);
    }

    with(num_reader, num_writer, || AtomicUsize::new(0), write, read);
}

// For performance reference only -- expect garbage results due to sync violations
pub fn with_unsynchronized(num_reader: usize, num_writer: usize) {
    fn write(counter: &Arc<usize>) {
        use std::borrow::Borrow;
        unsafe {
            *std::mem::transmute::<&usize, *mut usize>(Arc::borrow(counter)) += 1;
        }
    }

    fn read(counter: &Arc<usize>) {
        let _ = counter;
    }

    with(num_reader, num_writer, || usize::default(), write, read);
}
