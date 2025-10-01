use std::{sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc}, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    let (tx_stop, rx_stop) = mpsc::channel::<()>();

    let p2_stop = Arc::new(AtomicBool::new(false));
    let p2_stop_clone = Arc::clone(&p2_stop);

    let p1 = thread::spawn(move || {
        let mut counter = 0;
        loop {
            let msg = format!("Message {} from p1", counter);
            tx.send(msg).unwrap();
            counter += 1;
            thread::sleep(std::time::Duration::from_secs(1));

            match rx_stop.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    println!("p1 received stop signal, exiting.");
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    });
    let p2 = thread::spawn(move || {
        let mut counter = 0;
        loop {
            let msg = format!("Message {} from p2", counter);
            tx2.send(msg).unwrap();
            counter += 1;
            thread::sleep(std::time::Duration::from_secs(1));

            if p2_stop.load(Ordering::Relaxed) {
                println!("p2 received stop signal, exiting.");
                break;
            }
        }
    });

    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
        }
    });

    thread::sleep(std::time::Duration::from_secs(5));
    tx_stop.send(()).unwrap();
    p2_stop_clone.store(true, Ordering::Relaxed);
    
    p2.join().unwrap();   
    p1.join().unwrap();   
    consumer.join().unwrap();
}
