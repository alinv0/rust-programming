use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::types;
use types::Visit;

pub(crate) fn read_visits(filename: &String) -> Option<Vec<Visit>> {
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut visits: Vec<Visit> = Vec::new();
            for line in reader.lines() {
                let visit = Visit::from_string(line.unwrap());
                visits.push(visit);
            }

            Some(visits)
        }
        Err(e) => {
            println!("Error opening file: {}", e);
            None
        }
    }
}

pub(crate) fn write_visits_to_file(filename: &String, visits: &Vec<Visit>) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    for visit in visits {
        writer.write_all(visit.to_string().as_bytes()).unwrap();
    }
}