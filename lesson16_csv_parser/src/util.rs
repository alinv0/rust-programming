use chrono::NaiveDate;

pub fn parse_command_line() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        Some(args[1].clone())
    } else {
        None
    }
}

pub(crate) fn parse_date(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
}

pub fn prompt_for_string(message: &String) -> String {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn prompt_for_date(message: &String) -> NaiveDate {
    let input = prompt_for_string(message);
    let input = input.trim();
    parse_date(&input)
}