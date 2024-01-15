use std::sync::atomic::{AtomicI32, Ordering};

pub struct Employee {
    id: i32,
    name: String,
    salary: u64,
    fulltime: bool,
}

//Non-constant associated data
static NEXT_ID: AtomicI32 = AtomicI32::new(0);

impl Employee {
    //Constant associated data
    const MAX_SALARY: u64 = 99_000;
    pub fn payrise(&mut self, amount: u64) {
        self.salary += amount;
        if self.salary > Employee::MAX_SALARY {
            self.salary = Employee::MAX_SALARY;
        }
    }

    pub fn new(name: &str, salary: u64, fulltime: bool) -> Employee {
        Employee {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: name.to_string(),
            salary,
            fulltime,
        }
    }

    pub fn to_string(&self) -> String {
        format!("id: {}, name: {}, salary: {}, fulltime: {}", self.id, self.name, self.salary, self.fulltime)
    }
}