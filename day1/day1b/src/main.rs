use std::{fs, io};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();


    for line in input_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            // Parse and push each part into respective arrays
            list1.push(parts[0].parse::<i32>().unwrap());
            list2.push(parts[1].parse::<i32>().unwrap());
        }
    }
    
    let mut list2_counts = HashMap::new();
    for &num in &list2 {
        *list2_counts.entry(num).or_insert(0) += 1;
    }

    let mut sim_score = 0;
    for &num in &list1 {
        let count = list2_counts.get(&num).cloned().unwrap_or(0);
        sim_score += num*count;
    }
    println!("total sim score: {}", sim_score);
    Ok(())
}
