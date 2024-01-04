use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Utc};

pub fn do_it() {
    println!("\nIn demo_closures::do_it()");

    closure_noparams();
    closure_one_param();
    closure_many_params();
    closure_multiple_statements();
}

fn closure_noparams() {
    let get_timestamp = || -> DateTime<Utc> { Utc::now() };
    println!("get_timestamp() = {}", get_timestamp());
}

fn closure_one_param() {
    let reciprocal = |n: f64| -> f64 { if n == 0.0 { 0.0 } else { 1.0 / n } };
    println!("reciprocal(2.0) = {}", reciprocal(2.0));
}

fn closure_many_params() {
    let product = |x: i32, y: i32, z: i32| -> i32 { x * y * z };
    println!("product(2, 3, 4) = {}", product(2, 3, 4));
}

fn closure_multiple_statements() {
    let get_timestamp_after_delay = |seconds| -> DateTime<Utc> {
        sleep(Duration::new(seconds, 0));
        Utc::now()
    };

    println!("get_timestamp_after_delay(2) = {}", get_timestamp_after_delay(2).format("%T"));
}