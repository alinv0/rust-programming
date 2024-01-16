use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
struct EmpCode {
    code: String,
    country: String,
}

impl EmpCode {
    fn new(code: &str, country: &str) -> EmpCode {
        EmpCode {
            code: code.to_string(),
            country: country.to_string(),
        }
    }
}

#[derive(Debug)]
struct Emp {
    name: String,
    salary: f64,
}

impl Emp {
    fn new(name: &str, salary: f64) -> Emp {
        Emp {
            name: name.to_string(),
            salary,
        }
    }
}

pub fn do_it() {
    println!("\nIn demo_eq_hash::do_it()");

    let mut staff: HashMap<EmpCode, Emp> = HashMap::new();

    staff.insert(
        EmpCode::new("001", "IN"),
        Emp::new("John Doe", 1000.0),
    );

    staff.insert(
        EmpCode::new("002", "UK"),
        Emp::new("Jane Doe", 2000.0),
    );

    staff.insert(
        EmpCode::new("001", "US"),
        Emp::new("Joe Bloggs", 3000.0),
    );

    let emp = &staff[&EmpCode::new("002", "UK")];
    println!("emp: {:?}", emp);
}