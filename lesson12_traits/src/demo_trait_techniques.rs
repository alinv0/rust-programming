use crate::mystructs::employee::Employee;
use crate::mytraits::log::Log;
use crate::mytraits::print::Print;

pub fn do_it() {
    println!("\nIn demo_trait_techniques");

    let mut e1 = Employee::new(String::from("John"), 50000.0, true);
    e1.payrise(100.0);

    e1.print();
    e1.log_verbose();

    println!("Employee LOG_TIMESTAMP: {}", Employee::LOG_TIMESTAMP);
}