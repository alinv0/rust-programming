struct Point {
    x: i32,
    y: i32,
}

impl Point {

    // self: &Self
    // self: &Point
    fn print(&self) {
        println!("Point: x = {}, y = {}", self.x, self.y);
    }
}

pub fn do_it() {
    println!("\nIn demo_simple_impl::do_it()");

    let p1 = Point { x: 10, y: 20 };

    p1.print();
}