use crate::mystructs::employee::Employee;
use crate::mytraits::print::Print;

pub fn do_it() {
    println!("demo_trait_essentials::do_it()");

    let mut e1 = Employee::new(String::from("John"), 50000.0, true);
    e1.payrise(100.0);

    e1.print();
}