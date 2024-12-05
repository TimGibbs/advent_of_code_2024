use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let lines = input_data.lines();
    let mut parsing_tuples = true;

    for line in lines {
        if line.is_empty() {
            parsing_tuples = false;
            continue;
        }
        if parsing_tuples {
            if let Some((a, b)) = line.split_once('|') {
                let a: i32 = a.parse().expect("Invalid number");
                let b: i32 = b.parse().expect("Invalid number");
                rules.push((a, b));
            }
        }
        else {
            let array: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse().expect("Invalid number"))
                .collect();
            updates.push(array);
        }

    }

    let valid: Vec<Vec<i32>> = updates
        .into_iter()
        .filter(|update| check_update(update, &rules))
        .collect();
    
    let middles = valid.iter().map(|update| middle(update)).sum::<i32>();
    
    println!("{}", middles);
    Ok(())
}

fn check_update(update: &[i32], rules: &[(i32, i32)]) -> bool {

    for rule in rules {
        let index_a = update.iter().position(|&x| x == rule.0);
        let index_b = update.iter().position(|&x| x == rule.1);
        if index_a.is_some() && index_b.is_some() && index_b.unwrap() < index_a.unwrap() {
            return false;
        }
    }
    true
}

fn middle(update: &[i32]) -> i32 {
    let middle_index = update.len() / 2;
    update[middle_index]
}

