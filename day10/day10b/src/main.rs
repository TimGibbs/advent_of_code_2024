use std::{io};
use rayon::prelude::*;
use common::load_as_2d_char_matrix;
use common::add_vector_to_positions;
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
    iterate(matrix,position,0)
}

fn iterate(matrix: &[Vec<char>], position: (usize, usize), step:u32) -> usize {
    if position.0 >= matrix.len() || position.1 >= matrix[position.0].len() ||
        matrix[position.0][position.1].to_digit(10).unwrap() != step {
        return 0;
    }
    if step == 9 {
        return 1;
    }
    let directions = [
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 0),
    ];
    let chain : usize = directions.iter()
        .filter_map(|v| add_vector_to_positions(position,v))
        .map(|p| iterate(matrix,p,step+1))
        .sum();
    chain
}

