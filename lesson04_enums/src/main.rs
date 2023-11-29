mod mytypes;

use std::num::ParseIntError;
use mytypes::Color;
use crate::mytypes::HouseLocation;

fn main() {
    simple_enums();
    enums_with_data();
    option_enum();
    result_enum();
}

fn simple_enums() {
    let color: Color = Color::Red;

    match color {
        Color::Red => println!("Color is Red"),
        Color::Green => println!("Color is Green"),
        Color::Blue => println!("Color is Blue"),
    }
}

fn enums_with_data() {
    let h1 = HouseLocation::Number(123);
    let h2 = HouseLocation::Name(String::from("The White House"));
    let h3 = HouseLocation::Unknown;

    get_house_data(h1);
    get_house_data(h2);
    get_house_data(h3);
}

fn get_house_data(h: HouseLocation) {
    match h {
        HouseLocation::Number(n) => println!("House number is {}", n),
        HouseLocation::Name(s) => println!("House name is {}", s),
        HouseLocation::Unknown => println!("House location is unknown"),
    };
}

fn option_enum() {
    display_second_of_day(23, 59, 59);
    display_second_of_day(24, 0, 0);
    display_second_of_day(16, 59, 60);
    display_second_of_day(10, 60, 24);
}

fn result_enum() {
    let s = "123";
    let result = parse_string_to_int(s);
    print_parsed_result(s, result);

    let s = "cats and dogs";
    let result = parse_string_to_int(s);
    print_parsed_result(s, result);

    let s = "something";
    let result = parse_string_to_int(s);
    print_unwrapped_result(s,result);
}

fn print_parsed_result(s: &str, r: Result<i32, ParseIntError>) {
    match r {
        Ok(n) => println!("Parsed {} to integer {}", s, n),
        Err(e) => println!("Error parsing {}: {}", s, e)
    };
}

fn print_unwrapped_result(s: &str, r: Result<i32, ParseIntError>) {
    println!("Unwrapped {} to integer {}", s, r.unwrap_or(-1));
}

fn parse_string_to_int(s: &str) -> Result<i32, ParseIntError> {
    let res: Result<i32, std::num::ParseIntError>;
    res = i32::from_str_radix(s, 16);
    res
}

fn display_second_of_day(h: u32, m: u32, s: u32) {
    let sec: Option<u32> = sec_of_day(h, m, s);
    match sec {
        Some(s) => println!("Second of day: {}", s),
        None => println!("Invalid time"),
    }

    println!("Unwrapped second of day: {}", sec.unwrap_or(0));
}

fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> {
    if h <= 23 && m <= 59 && s <= 59 {
        Some(h * 3600 + m * 60 + s)
    } else {
        None
    }
}