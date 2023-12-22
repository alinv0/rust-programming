pub fn do_it() {
    println!("\nIn demo_locals::di_it()");

    let x = 42;

    if x != 0 {
        let s1 = "Alin";
        println!("s1: {}", s1);
    }

    // Won't work because s1 is out of scope
    // println!("s1: {}", s1);
}