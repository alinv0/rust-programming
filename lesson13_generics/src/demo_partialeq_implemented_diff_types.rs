#[derive(PartialEq, Debug)]
struct TimeSeconds {
    s: i32,
}

#[derive(PartialEq, Debug)]
struct TimeMinutes {
    m: i32,
}

impl PartialEq<TimeMinutes> for TimeSeconds {
    fn eq(&self, other: &TimeMinutes) -> bool {
        self.s == other.m * 60
    }
}

impl PartialEq<TimeSeconds> for TimeMinutes {
    fn eq(&self, other: &TimeSeconds) -> bool {
        self.m * 60 == other.s
    }
}

pub fn do_it() {
    println!("\nIn demo_partialeq_implemented::do_it()");

    let ts1 = TimeSeconds { s: 60 };
    let ts2 = TimeSeconds { s: 60 };
    let ts3 = TimeSeconds { s: 90 };
    println!("ts1: {:?}", ts1);
    println!("ts2: {:?}", ts2);
    println!("ts3: {:?}", ts3);

    let tm1 = TimeMinutes { m: 1 };
    let tm2 = TimeMinutes { m: 1 };
    let tm3 = TimeMinutes { m: 3 };
    println!("tm1: {:?}", tm1);
    println!("tm2: {:?}", tm2);
    println!("tm3: {:?}", tm3);

    println!("ts1 == ts2: {}", ts1 == ts2);
    println!("ts1.eq(&ts2): {}", ts1.eq(&ts2));
    println!("ts1 != ts3: {}", ts1 != ts3);
    println!("ts1.ne(&ts3): {}", ts1.ne(&ts3));

    println!("tm1 == tm2: {}", tm1 == tm2);
    println!("tm1.eq(&tm2): {}", tm1.eq(&tm2));
    println!("tm1 != tm3: {}", tm1 != tm3);
    println!("tm1.ne(&tm3): {}", tm1.ne(&tm3));

    println!("ts1 == tm1: {}", ts1 == tm1);
    println!("ts1.eq(&tm1): {}", ts1.eq(&tm1));
    println!("t1 != t3: {}", ts1 != ts3);
    println!("t1.ne(&t3): {}", ts1.ne(&ts3));
}
