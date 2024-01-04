pub fn do_it() {
    println!("\nIn demo_closures_capture_value::do_it()");

    capture_value_automatically();
    capture_value_forcibly();

    std::thread::sleep(std::time::Duration::new(10, 0));

}

fn capture_value_automatically() {
    let message = String::from("Hello");
    println!("message initially: {}", message);

    let consume_message = || {
        println!("message in closure: {}", message);
        std::mem::drop(message);
    };

    //Can't do this because message is moved into the closure
    //println!("message after closure: {}", message);

    //Can call the closure once
    consume_message();

    //Can't call the closure again
    //consume_message();
}

fn capture_value_forcibly() {
    let message = String::from("Hello");

    println!("Start of the method");

    std::thread::spawn(move || {
        println!("Message at the start of the method: {}", message);
        std::thread::sleep(std::time::Duration::new(5, 0));
        println!("Message at the end of the method: {}", message);
    });

    println!("End of the method");
}