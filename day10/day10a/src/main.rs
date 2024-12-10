use std::{io};
use std::collections::HashSet;
use rayon::prelude::*;
use common::load_as_2d_char_matrix;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let matrix = load_as_2d_char_matrix(file_path)?;
    
    let starts: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &cell)| if cell == '0' { Some((y, x)) } else { None })
        })
        .collect();

    let p: usize = starts.into_par_iter().map(|s| start_check(&matrix,s)).sum();

    println!("{}", p);
    
    Ok(())
}

fn start_check(matrix: &[Vec<char>], position: (usize, usize)) -> usize {
    if let Some(q) = iterate(matrix,position,0){
        let mut set = HashSet::new();
        for p in q {
            set.insert(p);
        }
        return set.len();
    };
    0
}

fn iterate(matrix: &[Vec<char>], position: (usize, usize), step:u32) -> Option<Vec<(usize, usize)>> {
    if position.0 >= matrix.len() || position.1 >= matrix[position.0].len() || 
        matrix[position.0][position.1].to_digit(10).unwrap() != step {
        return None;
    }
    if step == 9 {
        return Some(vec![position]);
    }
    let directions = [
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 0),
    ];
    let chain : Vec<(usize,usize)> = directions.iter()
        .filter_map(|v| add_vector_to_positions(position,v))
        .filter_map(|p| iterate(matrix,p,step+1))
        .flatten().collect();
    Some(chain)
}

fn add_vector_to_positions(position: (usize, usize), vector: &(isize, isize)) -> Option<(usize, usize)> {
    let y = add_isize_to_usize(position.0, vector.0)?;
    let x = add_isize_to_usize(position.1, vector.1)?;
    Some((y, x))
}

fn add_isize_to_usize(u: usize, i: isize) -> Option<usize> {
    if i >= 0 {
        u.checked_add(i as usize)
    } else {
        u.checked_sub((-i) as usize)
    }
}
