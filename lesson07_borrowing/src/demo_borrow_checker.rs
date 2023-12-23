pub fn do_it() {
    println!("\nIn demo_borrow_checker::do_it()");

    defining_many_immutable_references();
    restrictions_after_defining_mutable_reference();
    restrictions_after_defining_immutable_reference();
}

fn defining_many_immutable_references() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}

fn restrictions_after_defining_mutable_reference() {
    let mut s = String::from("hello");
    s.push_str(" world!");

    let r1 = &mut s;

    //let r2 = &mut s; // error
    //let r3 = &s; // error

    //You can't print s because it tries to borrow it immutably
    //println!("s: {}", s); // error

    r1.push_str(" ha ha ");

    println!("r1: {}", r1);
}

fn restrictions_after_defining_immutable_reference() {
    let mut s = String::from("hello");
    s.push_str(" world!");

    let r1 = &s;
    let r2 = &s;

    //no immutable borrows
    //let r3 = &mut s; // error

    //can't modify s
    //s.push_str(" ha ha "); // error

    println!("s: {}, r1: {}, r2: {}", s, r1, r2);

}
