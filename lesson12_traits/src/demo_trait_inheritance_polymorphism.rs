use crate::mystructs::employee::Employee;
use crate::mystructs::point::Point;
use crate::mytraits::print::Print;

pub fn do_it() {
    println!("\nIn demo_trait_inheritance_polymorphism");

    let obj1 = Employee::new(String::from("John"), 50000.0, true);
    let obj2 = Point::new(10, 20);
    print_something(&obj1);
    print_something(&obj2);

    //Create polymorphic collection
    let vec: Vec::<&dyn Print> = vec![&obj1, &obj2];

    for p in vec {
        p.print();
    }
}

//dynamic dispatch
fn print_something(p: &dyn Print) {
    p.print();
}