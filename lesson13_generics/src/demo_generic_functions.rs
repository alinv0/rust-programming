pub fn do_it() {
    println!("\nIn demo_generic_functions::do_it()");

    let ai = [10, 20, 30, 40, 50];
    process_array_ints(&ai);

    let af = [10.0, 20.0, 30.0, 40.0, 50.0];
    process_array_floats(&af);

    process_array(&ai);
    process_array(&af);
}

fn process_array<T>(p: &[T]) {
    println!("{} elements, {} bytes each",
             p.len(),
             std::mem::size_of::<T>());
}

fn process_array_floats(pf: &[f64; 5]) {
    println!("{} elements, {} bytes each",
             pf.len(),
             std::mem::size_of::<f64>());
}

fn process_array_ints(pi: &[i32; 5]) {
    println!("{} elements, {} bytes each",
             pi.len(),
             std::mem::size_of::<i32>());
}