use std::{
    time::Duration,
    sync::mpsc::channel
};
use std::thread;

fn main() {
    println!("Exchange of messages from more producers to one consumer");

    let (tx, rx) = channel();

    let tx_2 = tx.clone();      // Non posso passare la stessa variabile a più thread, ho bisogno di clonarla in una variabile tutta nuova
    let tx_3 = tx.clone();

    let transmitter_1 = thread::spawn(move || {
        let msg: Vec<String> = vec![
            String::from("Hi"),
            String::from("I"),
            String::from("am"),
            String::from("from"),
            String::from("England")
        ];

        for s in msg {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let transmitter_2 = thread::spawn(move || {
        let msg: Vec<String> = vec![
            String::from("Hola"),
            String::from("soy"),
            String::from("de"),
            String::from("España")
        ];

        for s in msg {
            tx_2.send(s).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let transmitter_3 = thread::spawn(move || {
        let msg: Vec<String> = vec![
            String::from("Cià"),
            String::from("song"),
            String::from("e"),
            String::from("Napl")
        ];

        for s in msg {
            tx_3.send(s).unwrap();
            thread::sleep(Duration::from_millis(1500));
        }
    });

    for m in rx {
        println!("Got: {}", m);
    }

    transmitter_1.join().unwrap();
    transmitter_2.join().unwrap();
    transmitter_3.join().unwrap();
}