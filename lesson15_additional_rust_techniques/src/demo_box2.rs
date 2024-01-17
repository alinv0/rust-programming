struct Employee {
    name: String,
    salary: u32,
}

pub fn do_it() {
    println!("In demo_box2::do_it()");

    let boxed_emp = Box::new(Employee {
        name: String::from("John Doe"),
        salary: 100_000,
    });

    // Pass the Box into a function.
    // This will move the Box ownership, as it does not implement the Copy trait
    process_employee(boxed_emp);

    // This won't compile because the Box has been moved
    // println!("boxed_emp = {:?}", boxed_emp);
}

fn process_employee(employee: Box<Employee>) {
    println!("Employee {} has salary {}", employee.name, employee.salary);
}