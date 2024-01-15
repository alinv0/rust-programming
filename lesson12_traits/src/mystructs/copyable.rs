#[derive(Copy, Clone, Debug)] //Copy works if each field implements Copy
pub struct Currency {
    pub dollars: i32,
    pub cents: i32,
}

impl Currency {
    pub fn new(dollars: i32, cents: i32) -> Currency {
        Currency { dollars, cents }
    }
}

//Alternative implementation:
// impl Copy for Currency {
//
// }
//
// impl Clone for Currency {
//     fn clone(&self) -> Currency {
//         println!("Cloning Currency with dollars: {} and cents: {}", self.dollars, self.cents);
//         Currency {
//             dollars: self.dollars,
//             cents: self.cents,
//         }
//     }
// }