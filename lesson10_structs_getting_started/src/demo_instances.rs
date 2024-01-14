use crate::mytypes::Employee;


pub fn do_it() {
    println!("\nIn demo_instances::do_it()");

    create_immutable_instance();
    create_mutable_instance();
}

fn create_immutable_instance() {
    let e1 = Employee {
        name: String::from("John Doe"),
        salary: 100_000,
        fulltime: true,
    };

    println!("{} earns {}, fulltime status: {}", e1.name, e1.salary, e1.fulltime);
}

fn create_mutable_instance() {
    let mut e1 = Employee {
        name: String::from("John Doe"),
        salary: 100_000,
        fulltime: true,
    };

    e1.salary = 110_000;

    println!("{} earns {}, fulltime status: {}", e1.name, e1.salary, e1.fulltime);
}