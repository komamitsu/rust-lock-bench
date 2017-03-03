use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{Ordering, AtomicUsize};

const N: i32 = 10000;

pub fn with<T, G>(num_reader: i32, num_writer: i32, gen: G, write: fn(&Arc<T>), read: fn(&Arc<T>))
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

pub fn with_mutex(num_reader: i32, num_writer: i32) {
    fn write(counter: &Arc<Mutex<i32>>) {
         let mut counter_ref = counter.lock().unwrap();
         *counter_ref += 1;
    }

    fn read(counter: &Arc<Mutex<i32>>) {
         let _ = counter.lock().unwrap();
    }

    with(num_reader, num_writer, || Mutex::new(0), write, read);
}

pub fn with_rwlock(num_reader: i32, num_writer: i32) {
    fn write(counter: &Arc<RwLock<i32>>) {
        let mut counter_ref = counter.write().unwrap();
        *counter_ref += 1;
    }

    fn read(counter: &Arc<RwLock<i32>>) {
        let _ = counter.read().unwrap();
    }

    with(num_reader, num_writer, || RwLock::new(0), write, read);
}

pub fn with_atomic(num_reader: i32, num_writer: i32) {
    fn write(counter: &Arc<AtomicUsize>) {
        counter.fetch_add(1, Ordering::SeqCst);
    }

    fn read(counter: &Arc<AtomicUsize>) {
        let _ = counter.load(Ordering::SeqCst);
    }

    with(num_reader, num_writer, || AtomicUsize::new(0), write, read);
}

