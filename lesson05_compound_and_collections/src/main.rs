use std::collections::HashMap;

fn main() {
    demo_arrays();
    demo_array_techniques();
    demo_tuples();
    demo_vectors();
    demo_maps();
}

fn demo_arrays() {
    println!("Using arrays");
    let a1 = [100, 101, 102];
    println!("a1 length is {:?}, first element is {:?}", a1.len(), a1[0]);

    //change items in a mutable array
    let mut a2 = [100, 101, 102];
    a2[1] = 999;
    println!("a2 length is {:?}, second element is {:?}", a2.len(), a2[1]);

    //print elements in a2
    for e in a2 {
        print!("{} ", e);
    }
}

fn demo_array_techniques() {
    println!("Array tecniques");

    let a1: [i64; 5];
    a1 = [100, 101, 102, 103, 104];
    println!("a1 is {:?}", a1);

    //You can fill an array with [filler; size], via the debug formatter
    let mut a2 = [99; 5];

    a2[0] = 58;
    a2[4] = 25;

    println!("a2 is {:?}", a2);
}

fn demo_tuples() {
    println!("Using tuples");

    let t1 = (9, "h1", 3.5); //fixed values
    println!("t1 elements are {}, {}, {}", t1.0, t1.1, t1.2);

    let mut t2 = (9, "h1", 3.5); //mutable values
    t2.0 = 10;
    println!("t2 elements are {}, {}, {}", t2.0, t2.1, t2.2);

    let t3 = (); //empty tuple
    println!("t3 is {:?}", t3);

    let t4: (i32, bool, f64);
    t4 = (58, true, 1.67);
    println!("t4 is {:?}, elements are {}, {}, {}", t4, t4.0, t4.1, t4.2);
}

fn demo_vectors() {
    println!("Using vectors");

    let mut _v1: Vec<i32> = Vec::new();
    let mut _v2 = Vec::<i32>::new();

    let mut v3 = vec![100, 101, 102];

    let item = v3[0];
    println!("Value: {}", item);

    let opt = v3.get(0);
    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    v3.push(103);
    v3.push(104);
    v3.push(105);
    v3.pop();
    v3.insert(0, 99);

    println!("Items in v3:");
    for item in &v3 {
        println!("{}", item);
    }
}

fn demo_maps() {
    println!("Using maps");

    let mut m: HashMap<String, i32> = HashMap::new();
    let mut _m2 = HashMap::<String, i32>::new();

    m.insert(String::from("UK"), 44);
    m.insert(String::from("NO"), 47);
    m.insert(String::from("SG"), 65);

    m.entry(String::from("SA")).or_insert(27);
    let val = m["UK"];
    println!("Value: {}", val);

    let opt = m.get("UK");
    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    println!("Items in m:");
    for entry in &m {
        println!(" {:?}", entry);
    }

    println!("m is {:?}", m);
}