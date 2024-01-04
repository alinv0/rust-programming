pub fn do_it() {
    println!("\nIn demo_nested_functions::do_it()");

    fn sqr(n: i32) -> i32 {
        n * n
    }

    println!("sqr(2) = {}", sqr(2));
    println!("sqr(3) = {}", sqr(3));
}