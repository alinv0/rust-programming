use chrono::NaiveDate;
use std::fmt::{Display, Formatter, Result};
use std::cmp::Ordering;
use crate::util;

pub struct Visit {
    start_date: NaiveDate,
    end_date: NaiveDate,
    description: String,
}

impl Visit {
    pub fn from_string(line: String) -> Visit {
        let tokens: Vec<&str> = line.split(",").collect();

        Visit {
            start_date: util::parse_date(tokens[0]),
            end_date: util::parse_date(tokens[1]),
            description: tokens[2].to_string(),
        }
    }

    pub fn get_days_in_period(&self, start_date: NaiveDate, end_date: NaiveDate) -> i64 {
        if self.start_date > end_date || self.end_date < start_date {
            0
        } else {
            let start = std::cmp::max(self.start_date, start_date);
            let end = std::cmp::min(self.end_date, end_date);
            end.signed_duration_since(start).num_days()
        }
    }
}

impl Display for Visit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{},{},{}\n", self.start_date, self.end_date, self.description)
    }
}

#[derive(Eq, PartialEq)]
pub struct Record {
    date: NaiveDate,
    days: i64,
}

impl Record {
    pub fn new(date: NaiveDate, days: i64) -> Record {
        Record { date, days }
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Days {}, date {}", self.days, self.date)
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        other.days.cmp(&self.days)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}