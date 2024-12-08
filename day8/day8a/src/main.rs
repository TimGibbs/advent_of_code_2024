use colored::*;
use std::{fs, io};
use std::collections::{HashMap, HashSet};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let matrix: Vec<Vec<char>> = input_data.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let y_limit = matrix.len();
    let x_limit = matrix[0].len();
    
    let mut positions_by_char: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row, line) in matrix.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch != '.' {
                positions_by_char.entry(ch).or_insert_with(Vec::new).push((row, col));
            }
        }
    }
    
    let mut total_uniques= HashSet::new();
    for x in positions_by_char.values() {
        let g = calculate_unique_antinodes(x);
        for q in  g.iter().filter(|&&(y,x)| y< y_limit && x< x_limit) { 
            total_uniques.insert(*q);
        }

    }
    
    for (row, line) in matrix.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if total_uniques.contains(&(row, col)) {
                print!("{}", ch.to_string().red());
            } else {
                print!("{}", ch);
            }
        }
        println!(); 
    }
    
    println!("{}",total_uniques.len());
    Ok(())
}

fn calc_antinodes(a: (usize, usize), b: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let d0 = a.0.abs_diff(b.0);
    let d1 = a.1.abs_diff(b.1);

    if a.0 >= b.0 {
        if a.1 >= b.1 {
            ((a.0 + d0, a.1 + d1), (b.0.wrapping_sub(d0), b.1.wrapping_sub(d1)))
        } else {
            ((a.0 + d0, a.1.wrapping_sub(d1)), (b.0.wrapping_sub(d0), b.1 + d1))
        }
    } else {
        if a.1 >= b.1 {
            ((a.0.wrapping_sub(d0), a.1 + d1), (b.0 + d0, b.1.wrapping_sub(d1)))
        } else {
            ((a.0.wrapping_sub(d0), a.1.wrapping_sub(d1)), (b.0 + d0, b.1 + d1))
        }
    }
    
}

fn calculate_unique_antinodes(points: &Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut unique_antinodes = HashSet::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];
            let (antinode1, antinode2) = calc_antinodes(a, b);

            // Insert both antinodes into the HashSet
            unique_antinodes.insert(antinode1);
            unique_antinodes.insert(antinode2);
        }
    }
    
    unique_antinodes
}