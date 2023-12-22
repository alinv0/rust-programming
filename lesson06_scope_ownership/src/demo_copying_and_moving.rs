pub fn do_it() {
    let a = 42;
    let b = a;
    println!("a: {}, b: {}", a, b);

    let s1 = String::from("hello");
    let s2 = s1;

    //error println("{}", s1);

    println!("s2: {}", s2);
}