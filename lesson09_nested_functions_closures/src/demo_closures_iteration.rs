pub fn do_it() {
    demo_simple_iteration();
    demo_unused_closure_variable();
    demo_filtering_mapping();
    demo_collecting_result();
}

fn demo_simple_iteration() {
    let v = vec!("ay", "vt", "ny", "ca", "or", "wa", "id", "nv", "ut", "az", "nm", "co", "wy", "mt");
    println!("Vector:");
    v.iter()
        .for_each(|s| println!("{}", s));
}

fn demo_unused_closure_variable() {
    let v = vec!("ay", "vt", "ny", "ca", "or", "wa", "id", "nv", "ut", "az", "nm", "co", "wy", "mt");
    println!("Vector:");
    v.iter()
        .for_each(|_| println!("111"));
}

fn demo_filtering_mapping() {
    let v = vec!("ay", "vt", "ny", "ca", "or", "wa", "id", "nv", "ut", "az", "nm", "co", "wy", "mt");
    println!("Vector:");
    v.iter()
        .filter(|s| s.ends_with("t"))
        .map(|s| s.to_uppercase())
        .for_each(|s| println!("{}", s));
}

fn demo_collecting_result() {
    let v = vec!("ay", "vt", "ny", "ca", "or", "wa", "id", "nv", "ut", "az", "nm", "co", "wy", "mt");
    println!("Vector:");
    let result = v.iter()
        .filter(|s| s.ends_with("t"))
        .map(|s| s.to_uppercase())
        .collect::<Vec<String>>();
    println!("Length: {}", result.len());

    result.iter()
        .for_each(|s| println!("{}", s));
}