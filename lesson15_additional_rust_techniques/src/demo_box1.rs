pub fn do_it() {
    println!("\nIn demo_box1::do_it()");

    // Create a Box object on the stack that points to data on the heap
    let b = Box::new(5);

    // Explicit dereference
    println!("b = {}", *b);
    // Implicit dereference
    println!("b = {}", b);

    // To use the value stored in b, we have to dereference it
    let x = *b;

    println!("x = {}", x);
}