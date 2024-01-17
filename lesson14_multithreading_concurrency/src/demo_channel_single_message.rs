use std::sync::mpsc;
use std::thread;

pub fn do_it() {
    println!("\nIn demo_channel_single_message::do_it()");

    // Create a channel
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        tx.send(String::from("Hello John Doe!")).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    handle.join().unwrap();
}