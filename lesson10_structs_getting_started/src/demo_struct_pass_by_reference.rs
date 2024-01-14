use crate::mytypes::Employee;

pub fn do_it() {
    println!("\nIn demo_struct_pass_by_reference::do_it()");

    let mut e1 = Employee {
        name: String::from("John Doe"),
        salary: 100_000,
        fulltime: true,
    };

    print_employee(&e1);

    reward_employee(&mut e1);

    print_employee(&e1);
}

fn print_employee(e: &Employee) {
    println!("Employee name: {}", e.name);
    println!("Employee salary: {}", e.salary);
    println!("Employee fulltime: {}", e.fulltime);
}

fn reward_employee(e: &mut Employee) {
    e.salary += 1000;
}