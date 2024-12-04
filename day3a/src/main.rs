use std::{fs, io};
use regex::{Regex};

fn main() -> io::Result<()> {
    let file_path = "data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let pattern = r"mul\((\d+),(\d+)\)";

    // Compile the regex
    let re = Regex::new(pattern).expect("Invalid regex");

    
    let mut sum = 0;
    // Find all matches
    for caps in re.captures_iter(&input_data) {
        // Extract the two captured groups
        let num1 = caps.get(1).unwrap().as_str();
        let num2 = caps.get(2).unwrap().as_str();

        sum += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }
    println!("{}", sum);
    Ok(())
}
