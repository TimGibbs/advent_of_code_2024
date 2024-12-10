use std::{fs, io};

pub fn load_as_2d_char_matrix(file_path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let input_data = fs::read_to_string(file_path)?;

    let matrix = input_data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    Ok(matrix)
}
