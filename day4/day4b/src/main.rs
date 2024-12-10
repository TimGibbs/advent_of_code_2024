use std::{io};
use common::load_as_2d_char_matrix;
fn main() -> io::Result<()> {
    let file_path = "../data.txt";

    let word_search= load_as_2d_char_matrix(file_path)?;
    let mut count = 0;

    for i in 1..word_search.len() -1 {
        for j in 1..word_search[i].len() -1  {
            if word_search[i][j] == 'A' {
                if matches_xmas(&word_search, i, j) {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn matches_xmas(
    word_search: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> bool {
    let a = word_search[i-1][j-1] == 'M' && word_search[i+1][j+1] == 'S';
    let b = word_search[i-1][j-1] == 'S' && word_search[i+1][j+1] == 'M';
    let c = word_search[i+1][j-1] == 'M' && word_search[i-1][j+1] == 'S';
    let d = word_search[i+1][j-1] == 'S' && word_search[i-1][j+1] == 'M';

    if (a||b) && (c||d) {
        return true;
    }
    false
}
