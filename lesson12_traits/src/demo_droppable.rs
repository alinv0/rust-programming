use crate::mystructs::droppable::Place;

pub fn do_it() {
    println!("\nIn demo_droppable::do_it()");

    let _p = Place::new("Boston", "USA");

    if true {
        let _p2 = Place::new("New York", "USA");
    }
    println!("Goodbye")
}