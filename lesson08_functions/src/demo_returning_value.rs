pub fn do_it() {
    let n = func_return_copyable_type();
    println!("n = {}", n);

    let s = func_return_noncopyable_type();
    println!("s = {}", s);
}

fn func_return_copyable_type() -> i32 {
    let n = 42;
    return n;
}

fn func_return_noncopyable_type() -> String {
    let s = String::from("Hello");
    return s;
}

// Does not work:
// fn bad_func_return_noncopyable_type() -> String {
//     let s = "Hello";
//     return s; // This line won't compile - s is a &str, not a String
// }