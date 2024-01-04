pub fn do_it() {
    println!("\nIn demo_returning_reference::do_it()");

    let s = String::from("Hello World!");

    let r1 = get_first_word(&s);
    println!("r1 = {}", r1);

    let r2: &str = get_first_word(&s);
    println!("r2 = {}", r2);

    let message: &str = get_message(99);
    println!("message = {}", message);
}

fn get_first_word(s: &str) -> &str {
    let mut pos = 0;
    for ch in s.chars() {
        if ch == ' ' {
            break;
        }
        pos += 1;
    }
    &s[..pos]
}

fn get_message(mark: i32) -> &'static str {
    if mark >= 50 {
        "PASS"
    } else {
        "FAIL"
    }
}

//bad functions

// Tries to return a reference to a local string
// fn bad_func1() -> &str {
//     let s = String::from("Hello");
//     &s
// }
//


// fn bad_func2(s: String) -> &str {
//     let mut pos = 0;
//     for ch in s.chars() {
//         if ch == ' ' {
//             break;
//         }
//         pos += 1;
//     }
//     &s[..pos]
// }