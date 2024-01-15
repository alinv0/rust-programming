use crate::mystructs::cloneable::Flight;

pub fn do_it() {
    println!("In demo_cloneable::do_it()");

    let f1 = Flight::new("Boston", "Dublin");
    let mut f2 = f1.clone();

    f2.redirect("Shanghai");

    println!("f1: {:?}", f1);
    println!("f2: {:?}", f2);
}