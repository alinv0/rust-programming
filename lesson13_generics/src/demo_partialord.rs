#[derive(PartialEq, PartialOrd, Debug)]
struct Currency {
    dollars: i32,
    cents: i32,
}

#[derive(Debug)]
struct Angle {
    degrees: i32,
}

impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self.degrees % 360 == other.degrees % 360
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<std::cmp::Ordering> {
        let d1 = self.degrees % 360;
        let d2 = other.degrees % 360;
        Some(d1.cmp(&d2))
    }
}

pub fn do_it() {
    println!("\nIn demo_partialord::do_it()");

    demo_derived();
    demo_impl()
}

fn demo_derived() {
    println!("In demo_derived()");

    let c1 = Currency { dollars: 3, cents: 75 };
    let c2 = Currency { dollars: 5, cents: 29 };
    let c3 = Currency { dollars: 9, cents: 75 };

    println!("c1: {:?}", c1);
    println!("c2: {:?}", c2);
    println!("c3: {:?}", c3);

    println!("c1 < c2: {}", c1 < c2);
    println!("c1.lt(&c2): {}", c1.lt(&c2));
    println!("c1 <= c2: {}", c1 <= c2);
    println!("c1.le(&c2): {}", c1.le(&c2));

    println!("c1 > c2: {}", c1 > c2);
    println!("c1.gt(&c2): {}", c1.gt(&c2));
    println!("c1 >= c2: {}", c1 >= c2);
    println!("c1.ge(&c2): {}", c1.ge(&c2));

    println!("c2 < c3: {}", c2 < c3);
    println!("c2 > c3: {}", c2 > c3);
}

fn demo_impl() {
    println!("In demo_impl()");

    let a1 = Angle { degrees: 90 };
    let a2 = Angle { degrees: 450 };
    let a3 = Angle { degrees: 180 };

    println!("a1: {:?}", a1);
    println!("a2: {:?}", a2);
    println!("a3: {:?}", a3);

    println!("a1 < a2: {}", a1 < a2);
    println!("a1.lt(&a2): {}", a1.lt(&a2));
    println!("a1 <= a2: {}", a1 <= a2);
    println!("a1.le(&a2): {}", a1.le(&a2));

    println!("a1 > a2: {}", a1 > a2);
    println!("a1.gt(&a2): {}", a1.gt(&a2));
    println!("a1 >= a2: {}", a1 >= a2);
    println!("a1.ge(&a2): {}", a1.ge(&a2));

    println!("a2 < a3: {}", a2 < a3);
    println!("a2 > a3: {}", a2 > a3);
}