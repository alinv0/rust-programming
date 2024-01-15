use crate::mytypes::employee::Employee;

pub fn do_it() {
    println!("\nIn demo_associated_data::do_it()");

    let e1 = Employee::new("John Doe", 10_000, false);
    println!("e1: {}", e1.to_string());

    let e2 = Employee::new("Jane Doe", 20_000, true);
    println!("e2: {}", e2.to_string());

    let e3 = Employee::new("Santa Claus", 30_000, true);
    println!("e3: {}", e3.to_string());

    let mut e4 = Employee::new("Rudolph Reindeer", 40_000, true);
    e4.payrise(100_000);
    println!("e4: {}", e4.to_string());
}