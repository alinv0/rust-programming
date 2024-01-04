pub fn do_it() {
    println!("\nIn demo_passing_references::do_it()");

    let n = 42;
    let s = String::from("Hello");
    some_func1(&n, &s);
    //cannot pass string slice to some_func1

    some_func2(&n, &s);
    some_func2(&n, "Hello World!");

    some_func3(&n, &s);
    some_func3(&n, "Hello John!");

    println!("n = {}", n);
    println!("s = {}", s);

    //some_func1(&n, s); //cannot pass string to some_func1
}

fn some_func1(iparam: &i32, sparam: &String) {
    if *iparam >= 50 {
        println!("{}, {}, PASS 😀", *iparam, (*sparam).to_uppercase());
    } else {
        println!("{}, {}, FAIL ??", *iparam, sparam.to_lowercase());
    }

    //It's OK to use (*param).func or param.func - it's the same.
}

fn some_func2(iparam: &i32, sparam: &str) {
    if *iparam >= 50 {
        println!("{}, {}, PASS 😀", *iparam, (*sparam).to_uppercase());
    } else {
        println!("{}, {}, FAIL ??", *iparam, sparam.to_lowercase());
    }

    //It's OK to use (*param).func or param.func - it's the same.
}

fn some_func3(iparam: &i32, sparam: &str) {
    println!("Values {0} and {1} addresses {0:p} and {1:p}", iparam, sparam);
}