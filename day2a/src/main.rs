use std::{fmt, fs};
use std::io;
#[derive(Debug)]
enum ParseError {
    InvalidNumber,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidNumber => write!(f, "Invalid number in the report"),
        }
    }
}

impl std::error::Error for ParseError {}

fn main() -> io::Result<()> {
    let file_path = "data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let reports: Vec<Vec<i32>> = parse_input(&input_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let safe_count = reports
        .iter()
        .filter(|report| check_report(report))
        .count();
    
    println!("Safe Count: {}", safe_count);
    Ok(())
}

/// Parses the input file into a vector of integer vectors.
fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, ParseError> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().map_err(|_| ParseError::InvalidNumber))
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect()
}

/// Checks if a report is safe. Returns `None` if the report is safe,
/// or `Some(index)` of the first failing element if it's unsafe.
fn check_report(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true; // A single number or empty report is considered safe
    }

    let increasing = report[0] < report[1];
    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];
        if increasing && (a < b && b - a <= 3) {
            continue;
        }
        if !increasing && (a > b && a - b <= 3) {
            continue;
        }
        return false; // The report is unsafe
    }

    true // The report is safe
}