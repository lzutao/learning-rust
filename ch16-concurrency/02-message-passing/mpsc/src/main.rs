use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // let tx1 = mpsc::Sender::clone(&tx);
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in ["hi", "from", "the", "thread"].iter() {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in ["more", "messages", "for", "you"].iter() {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {:?}", received);
    }
}
