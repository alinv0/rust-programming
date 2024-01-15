use chrono::{Utc};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    pub fn to_string(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }

    pub fn reset(&mut self) {
        self.log("reset");
        self.x = 0;
        self.y = 0;
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.log("move_by");
        self.x += dx;
        self.y += dy;
    }

    fn log(&self, msg: &str) {
        let ts = Utc::now().format("%T");
        println!("{}: x: {}, y: {}", msg, self.to_string(), ts);
    }
}