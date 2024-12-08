use rayon::prelude::*;
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
    
    let total_uniques: HashSet<(usize, usize)> = positions_by_char
        .values()
        .cloned() // Convert from `&Vec<(usize, usize)>` to `Vec<(usize, usize)>`
        .collect::<Vec<_>>()
        .par_iter()
        .flat_map(|x| {
            calculate_unique_antinodes(x, y_limit, x_limit)
        })
        .collect();
    
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

    println!("{}", total_uniques.len());
    Ok(())
}

fn calc_antinodes(
    a: (usize, usize),
    b: (usize, usize),
    y_limit: usize,
    x_limit: usize,
) -> Vec<(usize, usize)> {
    let d0 = a.0 as isize - b.0 as isize;
    let d1 = a.1 as isize - b.1 as isize;

    let mut antinodes = Vec::new();

    antinodes.push(a);

    // forward
    let mut current = (a.0, a.1);
    loop {
        let q0 = add_isize_to_usize(current.0, d0);
        let q1 = add_isize_to_usize(current.1, d1);
        if let (Some(y), Some(x)) = (q0, q1) {
            if y < y_limit && x < x_limit {
                antinodes.push((y, x));
                current = (y, x);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    // backwards
    let mut current = (a.0, a.1);
    loop {
        let q0 = add_isize_to_usize(current.0, -d0);
        let q1 = add_isize_to_usize(current.1, -d1);
        if let (Some(y), Some(x)) = (q0, q1) {
            if y < y_limit && x < x_limit {
                antinodes.push((y, x));
                current = (y, x);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    antinodes
}

fn calculate_unique_antinodes(
    points: &Vec<(usize, usize)>,
    y_limit: usize,
    x_limit: usize,
) -> HashSet<(usize, usize)> {
    let mut unique_antinodes = HashSet::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];
            let antinodes = calc_antinodes(a, b, y_limit, x_limit);

            // Insert all antinodes into the HashSet
            for antinode in antinodes {
                unique_antinodes.insert(antinode);
            }
        }
    }

    unique_antinodes
}

fn add_isize_to_usize(u: usize, i: isize) -> Option<usize> {
    if i >= 0 {
        u.checked_add(i as usize)
    } else {
        u.checked_sub((-i) as usize)
    }
}
