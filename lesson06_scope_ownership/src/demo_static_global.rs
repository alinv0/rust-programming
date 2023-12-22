use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

pub static GLOBAL_MESSAGE: &str = "My message ??";
static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
    let now = Utc::now();
    println!("global timestamp: {} initialized", now.format("%T"));
    return now
});

pub fn do_it() {
    println!("\nIn demo_static_global::di_it()");

    f1();
    f1();
    f2();
    f2();
}

fn f1() {
    println!("\n In f1, GLOBAL_MESSAGE: {}", GLOBAL_MESSAGE);
    println!("In f1, GLOBAL_TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));
}

fn f2() {
    println!("\n In f2, GLOBAL_MESSAGE: {}", GLOBAL_MESSAGE);
    println!("In f2, GLOBAL_TIMESTAMP: {}", (*GLOBAL_TIMESTAMP).format("%T"));
}