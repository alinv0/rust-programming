#[derive(Debug, Clone)] // Clone works if each field implements Clone
pub struct Flight {
    origin: String,
    destination: String,
}

impl Flight {
    pub fn new(origin: &str, destination: &str) -> Flight {
        Flight {
            origin: origin.to_string(),
            destination: destination.to_string(),
        }
    }

    pub fn redirect(&mut self, new_destination: &str) {
        self.destination = new_destination.to_string();
    }
}

// impl Clone for Flight {
//     fn clone(&self) -> Flight {
//         println!("Cloning Flight with origin: {} and destination: {}", self.origin, self.destination);
//         Flight {
//             origin: self.origin.clone(),
//             destination: self.destination.clone(),
//         }
//     }
// }