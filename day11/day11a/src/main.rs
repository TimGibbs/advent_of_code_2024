use std::{fs, io};
use regex::Regex;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let re = Regex::new(r"\d+").unwrap(); 
    let mut numbers: Vec<u128> = re.find_iter(&input_data)
        .filter_map(|mat| mat.as_str().parse::<u128>().ok())
        .collect();

    println!("{:?}", numbers);
    for _ in 0..25 {
        numbers = iterate(&numbers);
        println!("{:?}", numbers);
    }
    
    println!("{:?}", numbers.len());
    
    Ok(())
}

fn iterate(input: &[u128]) -> Vec<u128> {
    let mut ret : Vec<u128> = Vec::new();
    for i in 0..input.len() {
        if input[i] == 0 { ret.push(1); continue; }
        if total_digits(input[i])%2 == 0 { 
            let split = split_u128_in_half(input[i]);
            ret.push(split.0);
            ret.push(split.1);
            continue;
        }
        ret.push(input[i] * 2024);
    }
    ret
}

fn split_u128_in_half(number: u128) -> (u128, u128) {
    let number_str = number.to_string(); 
    let len = number_str.len();
    let mid = len / 2; 
    
    let (left, right) = number_str.split_at(mid);
    
    let left_part = left.parse::<u128>().unwrap_or(0);
    let right_part = right.parse::<u128>().unwrap_or(0);

    (left_part, right_part)
}

fn total_digits(number: u128) -> usize {
    if number == 0 {
        return 1
    }
    (number as f64).log10().floor() as usize + 1
}
