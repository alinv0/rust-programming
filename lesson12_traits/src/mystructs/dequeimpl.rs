use std::collections::VecDeque;
use crate::mytraits::collections::{Deque, Queue};

pub struct MyDeque {
    data: VecDeque<i32>,
}

impl MyDeque {
    pub fn new() -> MyDeque {
        MyDeque {
            data: VecDeque::new()
        }
    }
}

impl Queue for MyDeque {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn push_back(&mut self, value: i32) {
        self.data.push_back(value)
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

impl Deque for MyDeque {
    fn push_front(&mut self, value: i32) {
        self.data.push_front(value)
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }
}