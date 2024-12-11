use std::{fs, io};
use std::collections::HashMap;
use cached::proc_macro::cached;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let numbers: Vec<u128> = input_data
        .split_whitespace()
        .filter_map(|s| s.parse::<u128>().ok())
        .collect();
    
    let mut map = HashMap::<u128, u128>::new();
    for &num in &numbers {
        *map.entry(num).or_insert(0) += 1;
    }
    
    for i in 0..75 {
        map = iterate(map);
        println!("{:?}", i);
    }
    
    println!("{:?}", map.values().sum::<u128>());

    Ok(())
}

fn iterate(input: HashMap::<u128, u128>) -> HashMap::<u128, u128> {
    let mut response = HashMap::<u128, u128>::new();
    for (key, value) in &input {
        let result = handle_number(*key);
        match result {
            OneOrTwoNumbers::One(x) => {
                *response.entry(x).or_insert(0) += *value;
            }
            OneOrTwoNumbers::Two(x, y) => {
                *response.entry(x).or_insert(0) += *value;
                *response.entry(y).or_insert(0) += *value;
            }
        }
    }
    response
}


#[derive(Clone)]
enum OneOrTwoNumbers {
    One(u128),
    Two(u128, u128),
}

#[cached]
fn handle_number(number: u128) -> OneOrTwoNumbers {
    if number == 0 {
        OneOrTwoNumbers::One(1)
    } else if total_digits(number) % 2 == 0 {
        split_u128_in_half(number)
    } else {
        OneOrTwoNumbers::One(number * 2024)
    }
}

fn split_u128_in_half(number: u128) -> OneOrTwoNumbers {
    let num_digits = total_digits(number);
    let mid = num_digits / 2;
    let divisor = 10u128.pow(mid as u32);
    let left = number / divisor;
    let right = number % divisor;
    OneOrTwoNumbers::Two(left, right)
}

fn total_digits(number: u128) -> usize {
    if number == 0 {
        1
    } else {
        let mut digits = 0;
        let mut n = number;
        while n > 0 {
            n /= 10;
            digits += 1;
        }
        digits
    }
}
