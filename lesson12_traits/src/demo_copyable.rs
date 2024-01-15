use crate::mystructs::copyable::Currency;

pub fn do_it() {
    println!("\nIn demo_copyable::do_it()");

    let mut c1 = Currency::new(3, 75);
    let mut c2 = c1;

    c1.dollars = 100;
    c2.cents = 99;

    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);
}