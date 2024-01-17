use std::thread;
use std::time::Duration;

pub fn do_it() {
    println!("In demo_join_thread_single::do_it()");

    let handle = thread::spawn(|| {
        println!("{:?} starting", thread::current().id());

        thread::sleep(Duration::from_secs(10));
        println!("{:?} finishing", thread::current().id());

        //Uncomment to panic
        // panic!("oops!");
    });

    for i in 100..=105 {
        println!("{:?} displaying {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("{:?} waiting for other thread to end", thread::current().id());


    // Unsafe use of unwrap() will panic if the thread panics
    // handle.join().unwrap();

    // Safer use of unwrap()
    match handle.join() {
        Ok(_) => println!("joined successfully"),
        Err(_) => println!("panicked"),
    }
}

