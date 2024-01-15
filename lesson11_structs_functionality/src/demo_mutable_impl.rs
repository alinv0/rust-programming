struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn print(&self) {
        println!("Point is [{},{}]", self.x, self.y);
    }

    fn to_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

pub fn do_it() {
    println!("\nIn demo_mutable_impl::do_it()");

    let mut p1 = Point { x: 10, y: 20 };
    p1.print();

    p1.move_by(100, 100);

    println!("p1 is {}", p1.to_string());

    p1.reset();
    p1.print();
}