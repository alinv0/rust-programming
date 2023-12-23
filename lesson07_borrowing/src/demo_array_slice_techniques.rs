pub fn do_it() {
    println!("\nIn demo_array_slice_techniques::do_it()");

    slice_iteration();
    slice_part_of_array();
    slice_mutability();
}

fn slice_iteration() {
    let a = [10, 11, 12, 13, 14];
    let s1 = &a;

    println!("Elements in s1:");
    for elem in s1 {
        println!("  {}", elem);
    }
}

fn slice_part_of_array() {
    let a = [10, 11, 12, 13, 14];
    let s2 = &a[1..3];
    let s3 = &a[..3];
    let s4 = &a[2..];
    let s5 = &a[..];

    println!("\ns2 ptr: {:p}, len: {}, elems: {:?}", s2.as_ptr(), s2.len(), s2);
    println!("\ns3 ptr: {:p}, len: {}, elems: {:?}", s3.as_ptr(), s3.len(), s3);
    println!("\ns4 ptr: {:p}, len: {}, elems: {:?}", s4.as_ptr(), s4.len(), s4);
    println!("\ns5 ptr: {:p}, len: {}, elems: {:?}", s5.as_ptr(), s5.len(), s5);
}

fn slice_mutability() {
    let mut a = [10, 11, 12, 13, 14];
    let s = &mut a[2..4];
    s[0] = 99;
    println!("a: {:?}", a);
}
