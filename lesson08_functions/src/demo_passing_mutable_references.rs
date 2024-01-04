pub fn do_it() {
    println!("\nIn demo_passing_mutable_references::do_it()");

    let mut n = 42;
    let mut s = String::from("Hello");
    some_func(&mut n, &mut s);

    n += 1_000_000;
    s.push_str("👍 👍 👍");

    println!("n: {}", n);
    println!("s: {}", s);
}

fn some_func(iparam: &mut i32, sparam: &mut String) {
    println!("Values initially: {}, {}", iparam, sparam);
    *iparam += 10;
    sparam.push_str(" World!");
    println!("Values after: {}, {}", iparam, sparam);
}