use crate::mystructs::debuggable::Coord;

pub fn do_it() {
    println!("\nIn demo_debuggable::do_it()");

    let c = Coord::new(1.0, 2.0);
    println!("{:?}", c)
}