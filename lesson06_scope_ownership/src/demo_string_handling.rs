pub fn do_it() {
    println!("\nIn demo_string_handling::di_it()");

    using_string_literals();
    using_string_objects();
    using_mutable_string_objects();
}

fn using_string_literals() {
    let s1 = "hello";
    let s2: &'static str = "world";

    println!("s1: {}, ptr: {:p}, len: {}", s1, s1.as_ptr(), s1.len());
    println!("s2: {}, ptr: {:p}, len: {}", s2, s2.as_ptr(), s2.len());

}

fn using_string_objects() {
    let s3 = String::from("hello_object");
    let s4 = String::from("world_object");

    println!("s3: {}, ptr: {:p}, len: {}", s3, s3.as_ptr(), s3.len());
    println!("s4: {}, ptr: {:p}, len: {}", s4, s4.as_ptr(), s4.len());
}

fn using_mutable_string_objects() {
    let mut s5 = String::from("    hello");
    s5.push_str(" world!    ");

    let s6 = s5.trim();

    println!("s5: {}, ptr: {:p}, len: {}", s5, s5.as_ptr(), s5.len());
    println!("s6: {}, ptr: {:p}, len: {}", s6, s6.as_ptr(), s6.len());
}