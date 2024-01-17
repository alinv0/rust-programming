pub fn do_it() {
    println!("\nIn demo_unsafe_code::do_it()");

    let mut x = 100;
    x += 1;

    let mut y = 200;
    y += 1;

    let p1: *const i32 = &x;    //raw pointer that treats data as constant
    let p2: *mut i32 = &mut y;  //raw pointer that treats data as mutable


    //Dereference raw pointers using unsafe code. Won't work without unsafe code.
    unsafe {
        println!("p1: {}", *p1);

        //Won't work
        // *p1 = 111;
        // println!("p1: {}", *p1);

        *p2 = 222;
        println!("p2: {}", *p2);
    }
}