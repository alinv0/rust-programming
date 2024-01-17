use std::thread;
use std::time::Duration;

pub fn do_it() {
    println!("\nIn demo_capturing_state_implicit_move::do_it()");

    let handle = do_some_work();
    handle.join().unwrap();

    println!("All done!");
}

fn do_some_work() -> thread::JoinHandle<()> {
    let v = vec![100, 101, 102, 103, 104, 105];

    // The compiler infers how to capture v, depending on how it is used in the closure.
    // In this case, the closure takes ownership of v (move), because loop requires it.

    let handle = thread::spawn(move || {
        for i in v {
            println!("{} from spawned thread {:?}", i, thread::current().id());
            thread::sleep(Duration::from_millis(500));
        }
    });

    //This won't compile cause v has been moved to the closure
    // println!("v = {:?}", v);

    handle
}

