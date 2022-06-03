extern crate rand;
use rand::Rng;
use std::{
    thread,
    sync::{
        mpsc::sync_channel,
        Arc, RwLock
    },
    time::Duration
};

struct PairValues {
    l: u32,
    s: u32
}

impl PairValues {
    pub fn new (a: u32, b: u32) -> PairValues {
        PairValues {
            l: a,
            s: b
        }
    }
}

fn main() {
    println!("Reader/Writer in pipeline with Producer/Consumer");

    let mut handles = vec![];
    let (tx, rx) = sync_channel(4); // Canale con al massimo 5 messaggi
    let shared = Arc::new(RwLock::new(PairValues::new(0, 0)));

    let tx_gen = tx.clone();
    let generator = thread::spawn(move || {
        for _ in 0..10 {
            let b = PairValues::new(rand::thread_rng().gen_range(0..11), rand::thread_rng().gen_range(0..11));
            tx_gen.send(b).unwrap();
            println!("[SENDER] Message sent!");
        }
    });
    handles.push(generator);

    let shared_up = Arc::clone(&shared);
    let updater = thread::spawn(move || {
        for _m in rx {
            thread::sleep(Duration::from_millis(1000));

            {
                let mutex = &*shared_up;
                let mut lock = mutex.write().unwrap();
                // let mut _lock = cvar_s.wait_while(lock, |lock| lock.s > 0 || lock.l > 0).unwrap();
                // _lock.s += 1;
                *lock = _m;
            } // Qui viene liberato il mutex
            println!("[WRITER] Message received");
            println!("[WRITER] Buffer updated");
        }
    });
    handles.push(updater);

    for _ in 0..3 {
        let shared_dest = Arc::clone(&shared);
        let dest = thread::spawn(move || {
            for _ in 0..6 {
                thread::sleep(Duration::from_millis(2000));

                {
                    let mutex = &*shared_dest;
                    let lock = mutex.read().unwrap();
                    println!("[READER] a: {}, b: {}, a+b: {}", lock.l, lock.s, (lock.l + lock.s));
                }
            }
        });
        handles.push(dest);
    }

    for _h in handles {
        _h.join().unwrap();
    }

    println!("Finished");
}