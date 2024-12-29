use std::{fs, io};

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
    let instructions = parse_as_dir(&parse_as_dir(&parse_as_num(line)));
    
    Some(num * instructions.len())
    
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

fn parse_as_dir(input: &Vec<char>) -> Vec<char> {

    let mut ret = Vec::new();

    for i in 0..input.len() {
        let g = if i == 0 {dirpad('A',input[i])}
        else {dirpad(input[i-1],input[i])};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = parse_as_num(&vec!['0','2','9','A']);
        assert_eq!(result.len(), 12);
    }
    #[test]
    fn case2() {
        let result = parse_as_dir(&parse_as_num(&vec!['0','2','9','A']));
        assert_eq!(result.len(), 28);
    }
    #[test]
    fn case3() {
        let result = parse_as_dir(&parse_as_dir(&parse_as_num(&vec!['0','2','9','A'])));
        assert_eq!(result.len(), 68);
    }

    #[test]
    fn case4() {
        let result = parse_as_dir(&parse_as_dir(&parse_as_num(&vec!['9','8','0','A'])));
        assert_eq!(result.len(), 60);
    }

    #[test]
    fn case5() {
        let result = parse_as_dir(&parse_as_dir(&parse_as_num(&vec!['1','7','9','A'])));
        assert_eq!(result.len(), 68);
    }

    #[test]
    fn case6() {
        let result = parse_as_dir(&parse_as_dir(&parse_as_num(&vec!['4','5','6','A'])));
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn case7() {
        let result = parse_as_dir(&parse_as_dir(&parse_as_num(&vec!['3','7','9','A'])));
        assert_eq!(result.len(), 64);
    }
    
}