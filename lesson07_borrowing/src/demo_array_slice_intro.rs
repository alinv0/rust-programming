pub fn do_it() {
    println!("\nIn demo_array_slice_intro::do_it()");

    let a = [101, 102, 103, 104, 105];
    let s1 = &a;
    let s2: &[i32] = &a;

    println!("s1 ptr: {:p}, len: {}, elems: {:?}", s1.as_ptr(), s1.len(), s1);
    println!("s2 ptr: {:p}, len: {}, elems: {:?}", s2.as_ptr(), s2.len(), s2);
}
