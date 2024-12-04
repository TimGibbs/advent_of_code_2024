use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let word_search: Vec<Vec<char>> = input_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    
    let directions = [
        (0, 1),  // Horizontal right
        (0, -1),  // Horizontal left
        (1, 0),  // Vertical down
        (-1, 0),  // Vertical up
        (1, 1),  // Diagonal down-right
        (-1, 1),  // Diagonal up-right
        (1, -1),  // Diagonal down-left
        (-1, -1), // Diagonal up-left
    ];

    for i in 0..word_search.len() {
        for j in 0..word_search[i].len() {
            if word_search[i][j] == 'X' {
                for &(di, dj) in &directions {
                    if matches_xmas(&word_search, i, j, di, dj) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Number of XMAS occurrences: {}", count);
    Ok(())
}

// Helper function to check if XMAS exists in the given direction
fn matches_xmas(
    word_search: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    di: isize,
    dj: isize,
) -> bool {
    let letters = ['X','M', 'A', 'S'];
    for k in 1..letters.len() {
        let ni = i as isize + k as isize * di;
        let nj = j as isize + k as isize * dj;

        if ni < 0 || nj < 0 || ni as usize >= word_search.len() || nj as usize >= word_search[0].len()
        {
            return false; // Out of bounds
        }
        if word_search[ni as usize][nj as usize] != letters[k] {
            return false; // Character mismatch
        }
    }
    true
}