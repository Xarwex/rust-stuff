use std::cell::Cell;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = Cell::new(5);
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {:?}", received);
}
