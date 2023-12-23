pub fn do_it() {
    println!("\nIn demo_string_slice_intro::do_it()");

    slice_iteration();
    slice_part_of_string();
    slice_mutability();
}

fn slice_part_of_string() {
    let message = "howdy 😂";

    let s3 = &message[0..3];
    let s4 = &message[..3];
    let s5 = &message[2..5];
    let s6 = &message[2..];

    println!("\ns3 ptr: {:p}, len: {}, text: {}", s3.as_ptr(), s3.len(), s3);
    println!("\ns4 ptr: {:p}, len: {}, text: {}", s4.as_ptr(), s4.len(), s4);
    println!("\ns5 ptr: {:p}, len: {}, text: {}", s5.as_ptr(), s5.len(), s5);
    println!("\ns6 ptr: {:p}, len: {}, text: {}", s6.as_ptr(), s6.len(), s6);
}

fn slice_mutability() {
    let mut message = String::from("welcome");
    message.push_str(" home!");

    if true {
        let s: &mut str = &mut message[5..];
        s.make_ascii_uppercase();
    }

    println!("\nmessage: {}", message);
}

fn slice_iteration() {
    let s2 = "hello😂";
    println!("\nRaw bytes in s2 (in decimal, hex and octal):");

    for b in s2.bytes() {
        println!("  {} {:x} {:o}", b, b, b);
    }

    println!("\nChars in s2:");
    for ch in s2.chars() {
        println!("  {}", ch);
    }
}