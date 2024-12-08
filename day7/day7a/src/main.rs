use std::{fs, io};
use regex::Regex;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let lines : Vec<&str> = input_data.lines().collect();

    let mut sum : u128 = 0;
    for line in lines {
        let x = check_line(line);
        sum += x;
    }
    println!("{}", sum);
    Ok(())
}

fn check_line(line: &str) -> u128 {

    let operators : Vec<char> = Vec::from(['+', '*']);

    let re = Regex::new(r"\d+").unwrap(); // Matches one or more digits
    let numbers: Vec<u128> = re.find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<u128>().ok())
        .collect();

    let target = numbers[0];
    let components= &numbers[1..];
    let sequences = generate_operator_combinations(&operators, components.len()-1);

    for sequence in sequences {
        let mut result = components[0];
        for i in 0..sequence.len() {
            let char = sequence.chars().nth(i).unwrap();
            if char == '+' {
                result = result + components[i+1];
            }
            if char == '*' {
                result = result * components[i+1];
            }
        }
        if result == target { return target };
    }
    
    0
}

fn generate_operator_combinations(operators: &Vec<char>, slots: usize) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    backtrack(operators, slots, &mut current, &mut results);
    results
}

fn backtrack(operators: &Vec<char>, slots: usize, current: &mut Vec<char>, results: &mut Vec<String>) {
    if current.len() == slots {
        results.push(current.iter().collect());
        return;
    }

    for &op in operators {
        current.push(op);
        backtrack(operators, slots, current, results);
        current.pop();
    }
}
