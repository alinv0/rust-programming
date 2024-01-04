pub fn do_it() {
    capture_immutable_reference();
    capture_mutable_reference();
}

fn capture_immutable_reference() {
    let b1 = String::from("------->");
    let b2 = String::from("<-------");

    let display_message = |s| {
        println!("{}{}{}", b1, s, b2);
    };

    display_message(String::from("Hello"));
    display_message(String::from("World"));

    println!("b1 = {}", b1);
    println!("b2 = {}", b2);
}

fn capture_mutable_reference() {
    let mut b1 = String::from("------->");
    let mut b2 = String::from("<-------");

    let mut display_message = |s| {
        b1.push_str(" ]");
        b2.push_str(" [");

        println!("{}{}{}", b1, s, b2);
    };

    display_message(String::from("Hello"));
    display_message(String::from("World"));

    println!("b1 = {}", b1);
    println!("b2 = {}", b2);
}