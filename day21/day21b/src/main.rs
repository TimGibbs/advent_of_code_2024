use std::{fs, io};
use cached::proc_macro::cached;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let matrix : Vec<usize>= input_data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .filter_map(|line| handle_input_line(&line))
        .collect();

    let sum : usize = matrix.iter().sum();

    println!("{:?}", sum);
    
    Ok(())
}

fn handle_input_line(line:&Vec<char>) -> Option<usize> {
    let num = get_numeric_as_int(line)?;

    let instructions = parse_as_num(line);

    let mut sum = 0usize;
    
    for i in 0..instructions.len() {
        if i == 0 {
            sum+= handle_step('A',instructions[i], 24);
        }
        else {
            sum+= handle_step(instructions[i-1],instructions[i], 24 );
        };
    }
    
    Some(num * sum)

}

#[cached]
fn handle_step(from: char, to: char, steps_left:usize) -> usize {
    
    let controls = dirpad(from, to);
    if steps_left == 0 { return controls.len() }
    
    let mut sum = 0usize;
    
    for i in 0..controls.len() {
        if i == 0 {
            sum+= handle_step('A',controls[i], steps_left-1);
        }
        else {
            sum+= handle_step(controls[i-1],controls[i], steps_left-1);
        };
    }
    sum
}

fn parse_as_num(input: &Vec<char>) -> Vec<char> {

    let mut ret = Vec::new();

    for i in 0..input.len() {
        let g = if i == 0 {numpad('A',input[i])}
        else {numpad(input[i-1],input[i])};
        ret.extend(g);
    }

    ret
}

fn dirpad(from: char, to : char) -> Vec<char> {
    let mut s = match from {
        'A' => match to {
            'A' => vec![],
            '^' => vec!['<'],
            'v' => vec!['<', 'v'],
            '>' => vec!['v'],
            '<' => vec!['v', '<', '<'],
            _ => panic!("Invalid 'to' character!"),
        }
        '^' => match to {
            'A' => vec!['>'],
            '^' => vec![],
            'v' => vec!['v'],
            '>' => vec!['v', '>'],
            '<' => vec!['v', '<'],
            _ => panic!("Invalid 'to' character!"),
        }
        'v' => match to {
            'A' => vec!['^','>'],
            '^' => vec!['^'],
            'v' => vec![],
            '>' => vec!['>'],
            '<' => vec!['<'],
            _ => panic!("Invalid 'to' character!"),
        }
        '>' => match to {
            'A' => vec!['^'],
            '^' => vec!['<', '^'],
            'v' => vec!['<'],
            '>' => vec![],
            '<' => vec!['<', '<'],
            _ => panic!("Invalid 'to' character!"),
        }
        '<' => match to {
            'A' => vec!['>', '>', '^'],
            '^' => vec!['>', '^'],
            'v' => vec!['>'],
            '>' => vec!['>', '>'],
            '<' => vec![],
            _ => panic!("Invalid 'to' character!"),
        }
        _ => panic!("Invalid 'from' character!"),
    };

    s.push('A');
    s
}

fn numpad(from: char, to : char) -> Vec<char> {
    let mut s = match from {
        'A' => match to {
            'A' => vec![],
            '0' => vec!['<'],
            '1' => vec!['^', '<','<'],
            '2' => vec!['<', '^'],
            '3' => vec!['^'],
            '4' => vec!['^', '^','<','<'],
            '5' => vec!['<', '^', '^'],
            '6' => vec!['^', '^'],
            '7' => vec!['^', '^', '^', '<', '<'],
            '8' => vec!['<','^', '^', '^'],
            '9' => vec!['^', '^', '^'],
            _ => panic!("Invalid 'to' character!"),
        },
        '0' => match to {
            'A' => vec!['>'],
            '0' => vec![],
            '1' => vec!['^', '<'],
            '2' => vec!['^'],
            '3' => vec!['^', '>'],
            '4' => vec!['^', '^', '<'],
            '5' => vec!['^', '^'],
            '6' => vec!['^', '^', '>'],
            '7' => vec!['^', '^', '^', '<'],
            '8' => vec!['^', '^', '^'],
            '9' => vec!['^', '^', '^', '>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '1' => match to {
            'A' => vec!['>', '>', 'v'],
            '0' => vec!['>', 'v'],
            '1' => vec![],
            '2' => vec!['>'],
            '3' => vec!['>', '>'],
            '4' => vec!['^'],
            '5' => vec!['^', '>'],
            '6' => vec!['^', '>', '>'],
            '7' => vec!['^', '^'],
            '8' => vec!['^','^','>'],
            '9' => vec!['^', '^', '>', '>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '2' => match to {
            'A' => vec!['v','>'],
            '0' => vec!['v'],
            '1' => vec!['<'],
            '2' => vec![],
            '3' => vec!['>'],
            '4' => vec!['<','^'],
            '5' => vec!['^'],
            '6' => vec!['^', '>'],
            '7' => vec!['<','^', '^'],
            '8' => vec!['^', '^'],
            '9' => vec!['^', '^', '>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '3' => match to {
            'A' => vec!['v'],
            '0' => vec!['<', 'v'],
            '1' => vec!['<', '<'],
            '2' => vec!['<'],
            '3' => vec![],
            '4' => vec!['<', '<','^'],
            '5' => vec!['<','^'],
            '6' => vec!['^'],
            '7' => vec!['<', '<','^', '^'],
            '8' => vec!['<','^', '^'],
            '9' => vec!['^', '^'],
            _ => panic!("Invalid 'to' character!"),
        },
        '4' => match to {
            'A' => vec!['>', '>', 'v','v'],
            '0' => vec!['>','v', 'v'],
            '1' => vec!['v'],
            '2' => vec!['v', '>'],
            '3' => vec!['v', '>', '>'],
            '4' => vec![],
            '5' => vec!['>'],
            '6' => vec!['>', '>'],
            '7' => vec!['^'],
            '8' => vec!['^', '>'],
            '9' => vec!['^', '>', '>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '5' => match to {
            'A' => vec!['v', 'v','>'],
            '0' => vec!['v', 'v'],
            '1' => vec!['<','v'],
            '2' => vec!['v'],
            '3' => vec!['v', '>'],
            '4' => vec!['<'],
            '5' => vec![],
            '6' => vec!['>'],
            '7' => vec!['<','^'],
            '8' => vec!['^'],
            '9' => vec!['^', '>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '6' => match to {
            'A' => vec!['v', 'v'],
            '0' => vec!['<','v', 'v'],
            '1' => vec!['<', '<','v'],
            '2' => vec!['<','v'],
            '3' => vec!['v'],
            '4' => vec!['<', '<'],
            '5' => vec!['<'],
            '6' => vec![],
            '7' => vec!['<', '<','^'],
            '8' => vec!['<','^'],
            '9' => vec!['^'],
            _ => panic!("Invalid 'to' character!"),
        },
        '7' => match to {
            'A' => vec!['>', '>', 'v', 'v', 'v'],
            '0' => vec!['>','v', 'v', 'v'],
            '1' => vec!['v', 'v'],
            '2' => vec!['v', 'v', '>'],
            '3' => vec!['v', 'v', '>', '>'],
            '4' => vec!['v'],
            '5' => vec!['v', '>'],
            '6' => vec!['v', '>', '>'],
            '7' => vec![],
            '8' => vec!['>'],
            '9' => vec!['>', '>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '8' => match to {
            'A' => vec!['>', 'v', 'v', 'v'],
            '0' => vec!['v', 'v', 'v'],
            '1' => vec!['<','v', 'v'],
            '2' => vec!['v', 'v'],
            '3' => vec!['v', 'v', '>'],
            '4' => vec!['<','v'],
            '5' => vec!['v'],
            '6' => vec!['v', '>'],
            '7' => vec!['<'],
            '8' => vec![],
            '9' => vec!['>'],
            _ => panic!("Invalid 'to' character!"),
        },
        '9' => match to {
            'A' => vec!['v', 'v', 'v'],
            '0' => vec!['<','v', 'v', 'v'],
            '1' => vec!['<', '<','v', 'v'],
            '2' => vec!['<','v', 'v'],
            '3' => vec!['v', 'v'],
            '4' => vec!['<', '<','v'],
            '5' => vec!['<','v'],
            '6' => vec!['v'],
            '7' => vec!['<', '<'],
            '8' => vec!['<'],
            '9' => vec![],
            _ => panic!("Invalid 'to' character!"),
        },
        _ => panic!("Invalid 'from' character!"),
    };

    s.push('A');
    s
}

fn get_numeric_as_int(input: &Vec<char>) -> Option<usize> {
    let numeric_part: String = input.iter().filter(|c| c.is_numeric()).collect();
    numeric_part.parse::<usize>().ok() // Returns None if parsing fails
}
