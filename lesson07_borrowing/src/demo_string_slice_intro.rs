pub fn do_it() {
    println!("\nIn demo_string_slice_intro::do_it()");

    slice_string_object();
    slice_string_literal();
}

fn slice_string_object() {
    let obj = String::from("Hello");

    let s1 = &obj; // &String
    let s2: &str = &obj; //&str
}

fn slice_string_literal() {
    let s3 = "hello";
    let s4: &'static str = "world";

    println!("s3: {}, s4: {}", s3, s4);
}