use std::{fs, io};

pub fn load_as_2d_char_matrix(file_path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let input_data = fs::read_to_string(file_path)?;

    let matrix = input_data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    Ok(matrix)
}

pub fn add_isize_to_usize(u: usize, i: isize) -> Option<usize> {
    if i >= 0 {
        u.checked_add(i as usize)
    } else {
        u.checked_sub((-i) as usize)
    }
}

pub fn add_vector_to_positions(position: (usize, usize), vector: &(isize, isize)) -> Option<(usize, usize)> {
    let y = add_isize_to_usize(position.0, vector.0)?;
    let x = add_isize_to_usize(position.1, vector.1)?;
    Some((y, x))
}