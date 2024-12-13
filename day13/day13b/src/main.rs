use std::{fs, io};
use rayon::prelude::*;
#[derive(Debug)]
struct Data {
    a: (usize, usize),
    b: (usize, usize),
    target: (usize, usize),
}

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let parsed_data = parse_input(&input_data)
        .par_iter()
        .filter_map(|x| solve(x))
        .map(|x| x.0*3 + x.1)
        .sum::<usize>();

    println!("{}", parsed_data);


    Ok(())
}

fn parse_input(input: &str) -> Vec<Data> {
    let mut result = Vec::new();
    let mut a = (0, 0);
    let mut b = (0, 0);
    let mut target = (0, 0);

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            result.push(Data { a, b, target });
            continue;
        }

        if line.starts_with("Button A:") {
            let parts: Vec<&str> = line[9..].split(",").collect();
            a = (parse_value(parts[0]), parse_value(parts[1]));
        } else if line.starts_with("Button B:") {
            let parts: Vec<&str> = line[9..].split(",").collect();
            b = (parse_value(parts[0]), parse_value(parts[1]));
        } else if line.starts_with("Prize:") {
            let parts: Vec<&str> = line[6..].split(",").collect();
            target = (parse_value(parts[0])+10000000000000, parse_value(parts[1])+10000000000000);
        }
    }

    if a != (0, 0) || b != (0, 0) || target != (0, 0) {
        result.push(Data { a, b, target });
    }

    result
}

fn parse_value(part: &str) -> usize {
    let part = part.trim();
    let value: String = part
        .chars()
        .skip_while(|c| !c.is_digit(10) && *c != '-')
        .collect();
    value.parse::<usize>().unwrap()
}

fn solve(data: &Data) -> Option<(usize, usize)> {
    let a_numerator = data.b.1 as isize * data.target.0 as isize - data.b.0 as isize * data.target.1 as isize;
    let b_numerator = -(data.a.1  as isize) * data.target.0 as isize + data.a.0 as isize * data.target.1 as isize;
    let denominator = data.a.0 as isize * data.b.1 as isize - data.a.1 as isize * data.b.0 as isize;

    if denominator == 0 {
        return None; 
    }
    
    if a_numerator % denominator != 0 || b_numerator % denominator != 0 
        || a_numerator / denominator < 0 || b_numerator / denominator < 0  {
        return None; 
    }

    Some(((a_numerator / denominator) as usize, (b_numerator/denominator) as usize))
}