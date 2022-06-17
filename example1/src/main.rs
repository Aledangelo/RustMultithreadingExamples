/* Simple example where two thread work on the same resource */

use std::thread;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

fn main() {
    println!("This is a simple example with 2 threads working on the same vector 📂");
    println!();

    // Creo i miei dati li trasformo in mutex e li metto in ARC
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let data = Mutex::new(data);
    let data = Arc::new(data);

    // Creo un thread che aggiorna i valori
    // Un clone del nostro ARC sarà passato al thread
    // Clonando l'Arc abbiamo una reference al Mutex che contiene il vettore di dati
    // Quando cloniamo l'Arc e lo passiamo al thread, il clone sarà solo del thread in questione e non più
    // del thread principale
    let thread_data = data.clone();
    let thread_1 = thread::spawn(move || {
        println!("Thread 1 sta cercando di accedere alla risorsa...");
        // Il thread si bloccherà fin quando non potrà accedere ai dati
        // Il mutex può essere letto o modificato da un solo thread alla volta
        if let Ok(mut x) = thread_data.lock() {
            println!("Thread 1 ha acquisito la risorsa");
            for num in x.iter_mut() {
                *num += 1;
            }
            // Metto un po' di intalleo
            thread::sleep(Duration::from_millis(500));
        }
        println!("Thread 1 ha liberato la risorsa");

        thread::sleep(Duration::from_millis(750));
    });

    let thread_data = data.clone();
    let thread_2 = thread::spawn(move || {
        println!("Thread 2 sta cercando di accedere alla risorsa...");
        if let Ok(mut x) = thread_data.lock() {
            println!("Thread 2 ha acquisito la risorsa");
            for num in x.iter_mut() {
                *num += 2;
            }
            // Metto un po' di intalleo
            thread::sleep(Duration::from_millis(1000));
        }
        println!("Thread 2 ha liberato la risorsa");

        thread::sleep(Duration::from_millis(1250));
    });

    // Il thread principale aspetta che i due thread figli finiscano la loro esecuzione prima di stoppare tutto
    thread_1.join().unwrap();
    thread_2.join().unwrap();

    // A questo punto il thread principale acquisisce la risorsa e la estrae
    let data = data.lock().unwrap();

    // Poi la printa
    println!("Il valore finale è: {:?}", data);
}
