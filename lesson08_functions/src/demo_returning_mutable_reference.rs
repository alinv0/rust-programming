pub fn do_it() {
    println!("\nIn demo_returning_mutable_reference::do_it()");

    let mut s = String::from("Hello");
    let r = some_func(&mut s);

    r.push_str("!!!");
    println!("r = {}", r);
}

fn some_func(s: &mut String) -> &mut String {
    s.push_str(" World!");
    s
}

