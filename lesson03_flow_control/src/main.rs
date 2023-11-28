fn main() {
    demo_if();
    demo_match();
    demo_loop();
}

fn demo_if() -> () {
    let age = 58;
    if age > 50 {
        println!("You are old");
    } else if age > 30 {
        println!("You are middle aged");
    } else {
        println!("You are young");
    }

    let height = 1.8;
    if height > 1.8 {
        println!("You are tall");
    } else if height > 1.6 {
        println!("You are average height");
    } else {
        println!("You are short");
    }

    let msg = if age > 50 {"a"} else {"b"};
}

fn demo_match() {

    // simple match
    let num = 100;
    match num {
        100 => println!("Got 100"),
        200 => println!("Got 200"),
        _ => println!("Got something else"),
    }

    // match with range
    let num = 78;
    match num {
        25..=50 => println!("Got 25 to 50"),
        101..=200 => println!("Got 101 to 200"),
        _ => println!("Got something else"),
    }

    // match with multiple values
    let num = 23;
    match num {
        1 | 2 | 3 => println!("Got 1, 2 or 3"),
        10 | 11 | 23 => println!("Got 10, 11 or 23"),
        _ => println!("Got something else"),
    }

    // match with variable
    let num = 23;
    match num {
        n @ 1..=3 => println!("Got {} from 1 to 3", n),
        n @ 10..=23 => println!("Got {} from 10 to 23", n),
        _ => println!("Got something else"),
    }

    // match with if guard
    let num = 23;
    match num {
        x if x > 20 => println!("Got {} greater than 20", x),
        x if x < 20 => println!("Got {} less than 20", x),
        _ => println!("Got something else"),
    }

    let num = 40;
    let res = match num {
        x if x > 20 => "Got greater than 20",
        x if x < 20 => "Got less than 20",
        _ => "Got something else",
    };
    println!("res is {}", res);
}

fn demo_loop() {
    let mut i = 0;
    loop {
        println!("i is {}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }

    let mut i = 0;
    while i <= 10 {
        println!("i is {}", i);
        i += 1;
    }

    let arr = [10, 20, 30, 40, 50];
    for elem in arr {
        println!("elem is {}", elem);
    }

    for i in 0..=10 {
        println!("i is {}", i);
    }

    for i in 0..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i is {}", i);
    }

    let arr = [10, 20, 30, 40, 50];
    for i in arr.iter() {
        println!("i is {}", i);
    }

    let arr = [10, 20, 30, 40, 50];
    for (i, v) in arr.iter().enumerate() {
        println!("i is {} and v is {}", i, v);
    }
}

fn demo_break_and_continue() {

    // break
    let mut i = 0;
    loop {
        println!("i is {}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }

    // break outer loop
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }

    // continue
    let mut i = 0;
    while i <= 10 {
        println!("i is {}", i);
        i += 1;
    }

    // continue
    let arr = [10, 20, 30, 40, 50];
    for i in 0..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i is {}", i);
    }
}