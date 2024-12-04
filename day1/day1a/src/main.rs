use std::{fs, io};

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

    list1.sort();
    list2.sort();

    let mut dif = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        let abs_dif = i32::abs(a-b);
        dif += abs_dif;
    }
    println!("total dif: {}", dif);
    Ok(())
}