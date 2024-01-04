pub fn do_it() {
    println!("\nIn demo_passing_values::do_it()");

    let n = 42;
    let s = String::from("Hello");

    some_func(n, s);

    println!("n = {}", n);
    // println!("s = {}", s); // This line won't compile
}

fn some_func(n: i32, s: String) {
    println!("In some_func()");
    println!("n = {}", n);
    println!("s = {}", s);
}