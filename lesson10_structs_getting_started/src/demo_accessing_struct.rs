use crate::mytypes::Employee;

pub fn do_it() {
    println!("\nIn demo_accessing_struct::do_it()");

    let _e1: crate::mytypes::Employee;

    let _e2: Employee;

    let size = std::mem::size_of::<Employee>();
    println!("size of Employee = {}", size);
}