use std::fmt::Debug;

#[derive(Debug)]
pub struct Coord {
    long: f32,
    lat: f32,
}

impl Coord {
    pub fn new(long: f32, lat: f32) -> Coord {
        Coord { long, lat }
    }
}


//Alternative imoplementation:
/*
impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Coordinates: ({}, {})", self.long, self.lat)
    }

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Coord")
            .field("long", &self.long)
            .field("lat", &self.lat)
            .finish()
    }
}
*/