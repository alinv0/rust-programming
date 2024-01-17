use std::rc::Rc;

struct Employee {
    name: String,
    salary: f64,
}

pub fn do_it() {
    println!("demo_rc::do_it()...");

    let a = Rc::new(Employee { name: "Ion".to_string(), salary: 1000.0 });
    println!("Reference count initially: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("Reference count after creating b: {}", Rc::strong_count(&b));

    use_employee(&a);
    println!("Reference count after calling use_employee: {}", Rc::strong_count(&a));

    if true {
        let d = Rc::clone(&a);
        println!("Reference count inside the block: {}", Rc::strong_count(&d));
    }

    println!("Reference count after the block: {}", Rc::strong_count(&a));
}


fn use_employee(rc_emp: &Rc<Employee>) {
    let c = Rc::clone(rc_emp);
    println!("Reference count inside the function: {}", Rc::strong_count(&c));
}