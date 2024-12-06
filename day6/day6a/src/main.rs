use std::{fs, io};
use crate::Direction::{Down, Left, Right, Up};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let mut matrix: Vec<Vec<char>> = input_data.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut position = None;
    for (row_idx, row) in matrix.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == '^') {
            position = Some((row_idx, col_idx));
            break;
        }
    }
    let mut position = position.unwrap();
    let mut direction = Direction::Up;

    loop {
        // Mark current position as visited
        matrix[position.0][position.1] = 'X';

        // Get the next position
        if let Some(next_position) = get_next_position(&matrix, position, &direction) {
            let char = matrix[next_position.0][next_position.1];
            if char == '#' {
                direction = change_direction(direction);
                continue;
            }
            position = next_position;
        } else {
            break;
        }

        // // Print the updated matrix
        // for row in &matrix {
        //     println!("{:?}", row);
        // }
    }
    
    let count: usize = matrix.iter() // Iterate over each row
        .map(|row| row.iter().filter(|&&cell| cell == 'X').count()) // Count 'X' in each row
        .sum();
    
    println!("{}", count);
    Ok(())
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_next_position(
    matrix: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: &Direction
) -> Option<(usize, usize)> {
    let (row, col) = position;
    let next_position = match direction {
        Up => (row.wrapping_sub(1), col),
        Right => (row, col + 1),
        Down => (row + 1, col),
        Left => (row, col.wrapping_sub(1)),
    };

    // Check if the next position is within bounds
    if matrix.get(next_position.0).and_then(|r| r.get(next_position.1)).is_some() {
        Some(next_position)
    } else {
        None // Return None if out of bounds
    }
}

fn change_direction(direction: Direction) -> Direction {
    match direction {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}
