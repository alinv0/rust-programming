#[derive(Debug)]
struct Coordinate<T> {
    x: T,
    y: T,
    z: T,
}

pub fn do_it() {
    println!("\nIn demo_generic_structs::do_it()");

    //explicitly specify the type
    let c1 = Coordinate::<i32> { x: 1, y: 2, z: 3 };

    //let the compiler infer the type
    let c2 = Coordinate { x: 1.1, y: 2.2, z: 3.3 };
    let c3 = Coordinate { x: "x", y: "y", z: "z" };

    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);
    println!("c3: {:?}", c3);
}