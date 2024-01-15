use chrono::Utc;

pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn print(&self) {
        println!("x: {}, y: {}, z: {}", self.x, self.y, self.z);
    }

    pub fn to_string(&self) -> String {
        format!("x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }

    pub fn reset(&mut self) {
        self.log("reset");
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, dz: i32) {
        self.log("move_by");
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    fn log(&self, msg: &str) {
        let ts = Utc::now().format("%T");
        println!("at: {}, from: {}, there is msg: {}", ts, self.to_string(), msg);
    }
}