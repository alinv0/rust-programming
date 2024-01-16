use std::fmt::Debug;

pub fn do_it() {
    println!("\nIn demo_type_constraints::do_it()");

    let ai = [1, 2, 3];
    let af = [1.1, 2.2, 3.3];

    display_array(&ai);
    display_array(&af);
}

//This function will only work. T must implement the Debug trait
// fn display_array_bad<T>(arr: &[T]) {
//     for item in arr {
//         println!("{:?}", item);
//     }
// }


fn display_array<T: Debug>(arr: &[T]) {
    for item in arr {
        println!("{:?}", item);
    }
}