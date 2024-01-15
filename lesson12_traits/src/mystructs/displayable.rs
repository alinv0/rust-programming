use std::fmt::{Display, Formatter, Result};

pub struct Coord {
    long: f32,
    lat: f32,
}

impl Coord {
    pub fn new(long: f32, lat: f32) -> Coord {
        Coord { long, lat }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Coordinates: ({}, {})", self.long, self.lat)
    }

    //Alternative imoplementation:
    // fn fmt(&self, f: &mut Formatter) -> Result {
    //     f.debug_struct("Coord")
    //         .field("long", &self.long)
    //         .field("lat", &self.lat)
    //         .finish()
    // }
}