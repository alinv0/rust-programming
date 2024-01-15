use crate::mystructs::dequeimpl::MyDeque;
use crate::mytraits::collections::{Deque, Queue};

pub fn do_it() {
    println!("\nIn demo_trait_inheritance::do_it()");

    let mut d = MyDeque::new();

    d.push_back(10);
    d.push_back(50);
    d.push_back(30);
    d.push_front(100);
    d.push_front(200);

    println!("Deque length: {}", d.len());

    loop {
        match d.pop_front() {
            Some(v) => println!("Deque value: {}", v),
            None => break
        }
    }
}