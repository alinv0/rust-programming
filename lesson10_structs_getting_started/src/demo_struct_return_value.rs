use crate::mytypes::Employee;

pub fn do_it() {
    println!("\nIn demo_struct_return_value::do_it()");

    let e1 = build_employee(String::from("John Doe"), 100_000, true);
    let mut e2 = build_employee(String::from("Jane Doe"), 100_000, true);

    println!("{} earns {}, fulltime status: {}", e1.name, e1.salary, e1.fulltime);
    println!("{} earns {}, fulltime status: {}", e2.name, e2.salary, e2.fulltime);
    e2.salary = 110_000;
    println!("{} earns {}, fulltime status: {}", e2.name, e2.salary, e2.fulltime);
}

fn build_employee(name: String,
                  salary: u64,
                  fulltime: bool) -> Employee {
    Employee { name, salary, fulltime }
}