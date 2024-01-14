use crate::mytypes::Employee;

pub fn do_it() {
    println!("\nIn demo_struct_return_reference::do_it()");
    let mut e1 = Employee {
        name: String::from("John Doe"),
        salary: 100_000,
        fulltime: true,
    };
    let mut e2 = Employee {
        name: String::from("Jane Doe"),
        salary: 110_000,
        fulltime: true,
    };
    let e3 = choose_employee(&e1, &e2);

    println!("e3: {} earns {}, fulltime status: {}", e3.name, e3.salary, e3.fulltime);

    let e4 = choose_mut_employee(&mut e1, &mut e2);
    e4.salary = 0;
    println!("e4: {} earns {}, fulltime status: {}", e4.name, e4.salary, e4.fulltime);

}

fn choose_employee<'a>(e1: &'a Employee, e2: &'a Employee) -> &'a Employee {
    if e1.salary > e2.salary {
        e1
    } else {
        e2
    }
}

fn choose_mut_employee<'a>(e1: &'a mut Employee, e2: &'a mut Employee) -> &'a mut Employee {
    if e1.salary > e2.salary {
        e1
    } else {
        e2
    }
}