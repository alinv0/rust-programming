use crate::mytraits::log::Log;
use crate::mytraits::print::Print;

pub struct Employee {
    name: String,
    salary: f64,
    permanent: bool,
}

impl Employee {
    pub fn payrise(&mut self, amount: f64) {
        self.salary += amount;
    }

    pub fn new(name: String, salary: f64, permanent: bool) -> Employee {
        Employee { name, salary, permanent }
    }
}

impl Print for Employee {
    fn print(&self) {
        println!("name: {}, salary: {}, permanent: {}", self.name, self.salary, self.permanent);
    }
}

impl Log for Employee {
    const LOG_TIMESTAMP: bool = true;
    fn log(&self) {
        println!("name: {}, salary: {}, permanent: {}", self.name, self.salary, self.permanent);
    }
}