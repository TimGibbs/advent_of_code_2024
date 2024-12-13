use std::{fs, io};

#[derive(Debug)]
struct Data {
    a: (i32, i32),
    b: (i32, i32),
    target: (i32, i32),
}

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let parsed_data = parse_input(&input_data)
        .iter()
        .filter_map(|x| play_game(x))
        .map(|x| x.0*3 + x.1)
        .sum::<i32>();
    
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
            target = (parse_value(parts[0]), parse_value(parts[1]));
        }
    }

    if a != (0, 0) || b != (0, 0) || target != (0, 0) {
        result.push(Data { a, b, target });
    }

    result
}

fn parse_value(part: &str) -> i32 {
    let part = part.trim();
    let value: String = part
        .chars()
        .skip_while(|c| !c.is_digit(10) && *c != '-')
        .collect();
    value.parse::<i32>().unwrap()
}

fn play_game(data: &Data) -> Option<(i32, i32)> {
    let limit = 100;

    for a in  0..limit+1 {
        let a_total = (a * data.a.0, a * data.a.1);
        for b in 0..limit+1 {
            let b_total = (b * data.b.0, b * data.b.1);
            if a_total.0 + b_total.0 == data.target.0 
                && a_total.1 + b_total.1 == data.target.1 {
                return Some((a, b));
            }
        }
    }
    None

}