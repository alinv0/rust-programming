use crate::mystructs::displayable::Coord;

pub fn do_it() {
    println!("\nIn demo_displayable::do_it()");

    let c = Coord::new(1.0, 2.0);
    println!("{}", c)
}