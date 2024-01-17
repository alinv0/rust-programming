use std::sync::mpsc;
use std::thread;

pub fn do_it() {
    println!("\nIn demo_channel_multiple_messages::do_it()");

    // Create a channel
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for i in 1..=5 {
            let msg = format!("Message {}", i);
            println!("Sending: {}", msg);
            tx.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    handle.join().unwrap();
}