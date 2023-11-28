fn main() {
    println!("Hello, world!");
    demo_integers();
    demo_floats();
    demo_other_simple_types();
    demo_additional_techniques();
}

fn demo_integers() {
    /*
    Integer Types i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
     */

    // Rust has signed integers
    let a1: i32 = 10;
    let a2: i32 = 0xFFFF;
    let a3: i32 = 0o777;
    let a4: i32 = 0b1111_1111;

    let b: u32 = 12132;

    // Rust has architecture-dependent isize and usize types
    let c: isize = 101221;

    println!("c size: {}", std::mem::size_of_val(&c));
}

fn demo_floats() {
    /*
    Floating-Point Types f32, f64
     */

    let f1: f32 = 1.23456;
    println!("f1 size: {:.2}", f1);
    println!("f1 size: {:~>10.3}", f1);
    println!("f1 size: {:~<10.2}", f1);

    let f3: f32 = -1.60217657e-19;
    let f4: f64 = 2.71828182845e8;

    // Print the value of f3 and f4 in scientific notation
    println!("f3 = {:e}", f3);
    println!("f4 = {:e}", f4);
}

fn demo_other_simple_types() {
    let is_welsh: bool = true;
    let can_sing: bool = false;
    let is_welsh_num: i32 = is_welsh as i32;
    let can_sing_num: i32 = can_sing as i32;

    let middle_c: char = 'C';
    let heart_eyed_cat: char = '😻';
}

fn demo_additional_techniques() {
    let a = -12345;
    let b = 3.14;
    let c = 'X';

    println!("a is {} and b is {} and c is {}", a, b, c);

    let d = 0;
    // d = 1; // Error: cannot assign twice to immutable variable
    println!("d is {}", d);

    let mut e = 0;
    println!("e is {}", e);
    e = 1;
    println!("e is {}", e);

    let _f = 0;

    let g = 3.55;
    let h = g as i32;
    println!("g is {} and h is {}", g, h);

    let num = "231231";
    println!("num is {}", num);
    let num = 12345;
    println!("num is {}", num);

    const PI: f64 = std::f64::consts::PI;
    println!("PI is {}", PI);
}