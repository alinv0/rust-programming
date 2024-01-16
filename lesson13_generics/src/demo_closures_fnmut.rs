fn receive_fnmut<F>(mut func: F) where F: FnMut() {
    func();
    func();
}

pub fn do_it() {
    println!("\nIn demo_closures_fnmut::do_it()");

    // Can't pass in an FnOnce closure
    // let s1 = String::from("aaa");
    // receive_fnmut(|| {
    //     println!("s1: {}", s1);
    //     drop(s1);
    // });

    // Can pass in an FnMut closure
    let mut s2 = String::from("bbb");
    receive_fnmut(|| {
        s2.push_str("ccc");
        println!("s2: {}", s2);
    });

    // Can pass in an Fn closure
    let s3 = String::from("ddd");
    receive_fnmut(|| {
        println!("s3: {}", s3);
    });
}

