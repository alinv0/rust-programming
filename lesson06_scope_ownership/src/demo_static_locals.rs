use once_cell::sync::Lazy;
use chrono::{DateTime, Utc};

use std::thread::sleep;
use std::time::Duration;

pub fn do_it() {
    println!("\nIn demo_static_locals::di_it()");

    static_init_at_compile_time();
    static_init_at_runtime();
}

fn static_init_at_compile_time() {
    static MESSAGE: &str = "My message 😇";
    println!("MESSAGE: {}", MESSAGE);
}

fn static_init_at_runtime() {
    //You can't create a static variable at runtime, but you can create a static variable
    // static TIMESTAMP_WONT_WORK: DateTime<Utc> = Utc::now();
    println!("Current time: {}", Utc::now().format("%T"));

    static TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
        sleep(Duration::new(5, 0));
        let now = Utc::now();
        println!("Current time: {}", now.format("%T"));
        return now;
    });

    println!("TIMESTAMP: {}", (*TIMESTAMP).format("%T"));
}