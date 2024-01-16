fn receive_fn<F>(func: F) where F: Fn() {
    func();
}

pub fn do_it() {
    println!("\nIn demo_closures_fn::do_it()");

    // Can't pass in an FnOnce closure
    // let s1 = String::from("aaa");
    // receive_fn(|| {
    //     println!("s1: {}", s1);
    //     drop(s1);
    // });

    // Can't pass in an FnMut closure
    // let mut s2 = String::from("bbb");
    // receive_fn(|| {
    //     s2.push_str("ccc");
    //     println!("s2: {}", s2);
    // });

    // Can pass in an Fn closure
    let s3 = String::from("ddd");
    receive_fn(|| {
        println!("s3: {}", s3);
    });
}

