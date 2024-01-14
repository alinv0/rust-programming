use crate::mytypes::Employee;

pub fn do_it() {
    println!("\nIn demo_struct_pass_by_value::do_it()");

    let e1 = Employee {
        name: String::from("John Doe"),
        salary: 100_000,
        fulltime: true,
    };


    //Pass struct by value and move ownership to the function
    consume_employee(e1);

    //Can't use e1 here because it was moved into the function
}

fn consume_employee(e: Employee) {
    println!("Employee name: {}", e.name);
    println!("Employee salary: {}", e.salary);
    println!("Employee fulltime: {}", e.fulltime);
}