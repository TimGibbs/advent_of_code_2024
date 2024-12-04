use std::fs;
use std::io;
use std::fmt;

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
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let reports: Vec<Vec<i32>> = parse_input(&input_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let safe_count = reports
        .iter()
        .filter(|report| is_safe_with_retries(report))
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

/// Attempts to fix a report by removing a single element at `index_to_skip`.
/// Returns `true` if the modified report is safe.
fn is_safe_after_modification(report: &[i32], index_to_skip: usize) -> bool {
    let modified_report: Vec<_> = report
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != index_to_skip)
        .map(|(_, &v)| v)
        .collect();

    check_report(&modified_report)
}

/// Checks if a report is safe, allowing for one level to be removed
fn is_safe_with_retries(report: &[i32]) -> bool {
    if check_report(report) {
        return true;
    }
    for i in 0..report.len() {
        if is_safe_after_modification(report, i) {
            return true;
        }
    }
    false
}
