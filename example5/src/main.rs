use std::{
    time::Duration,
    thread,
    sync::{Arc, Mutex, Condvar},
    convert::TryFrom
};
extern crate rand;
use rand::Rng;

struct Buffer {
    arr: [i32; 5],
    n_elem:  u32
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            arr: [0; 5],
            n_elem: 0
        }
    }
}

fn main() {
    println!("Multiple Producer - Multiple Consumer");

    let data = Arc::new((Mutex::new(Buffer::new()), Condvar::new(), Condvar::new()));
    let mut handles = vec![];

    for _ in 0..15 {
        let cdata = Arc::clone(&data);
        let prod = thread::spawn(move || {
            for _ in 0..10 {
                let (lock, cvar_p, cvar_c) = &*cdata;
                let b = lock.lock().unwrap();

                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100, 300)));

                let mut _b = cvar_p.wait_while(b, |b| b.n_elem == 5).unwrap();

                let index = _b.n_elem;
                let n: i32 = rand::thread_rng().gen_range(0, 50);
                let index_us = usize::try_from(index).unwrap();
                println!("Insert -> {}", n);
                _b.arr[index_us] = n;
                _b.n_elem += 1;

                cvar_c.notify_all();
            }
        });
        handles.push(prod);

        let cdata = Arc::clone(&data);
        let cons = thread::spawn(move || {
            for _ in 0..5 {
                let (lock, cvar_p, cvar_c) = &*cdata;
                let b = lock.lock().unwrap();

                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(500, 2000)));

                let mut _b = cvar_c.wait_while(b, |b| b.n_elem == 0).unwrap();

                _b.n_elem -= 1;
                let index = _b.n_elem;
                let index_us = usize::try_from(index).unwrap();
                let num = _b.arr[index_us];
                println!("Extracting -> {}", num);

                cvar_p.notify_all();
            }
        });

        handles.push(cons);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
