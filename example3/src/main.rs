use std::sync::mpsc;    // mpsc = Multiple Producers Single Consumer
use std::thread;
use std::time::Duration;

fn main() {
    println!("Exchange of messages using channels");

    let (tx, rx) = mpsc::channel();    // Creo il canale, come valore di ritorno ottengo un mittente ed un ricevente

    thread::spawn(move || {
        // let msg: String = String::from("Hello Friend");
        let msg: Vec<String> = vec![
            String::from("Hello"),
            String::from("Friend"),
            String::from(""),
            String::from("Welcome"),
            String::from("to"),
            String::from("fsociety"),
            String::from("📟"),
            String::from(""),
            String::from("Wait"),
            String::from("for"),
            String::from("instructions")
        ];
        // tx.send(msg).unwrap();
        for s in msg {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(750));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    /* If there is just one message
    let received: String = rx.recv().unwrap();
    println!("Got Message: {}", received);
    */
}