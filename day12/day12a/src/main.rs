use std::{io};
use std::collections::HashSet;
use common::load_as_2d_char_matrix;
use common::add_vector_to_positions;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let matrix = load_as_2d_char_matrix(file_path)?;

    let mut total_sum = 0;
    
    let mut visited : HashSet<(usize,usize)> = HashSet::new();
    
    for i  in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if !visited.contains(&(i,j)) {
                let (area,perimeter) = walk(&matrix,(i,j),&mut visited);
                total_sum += area*perimeter;
                println!("{} a:{}, p:{}", matrix[i][j] , area,perimeter);
            }
        }
    }
    println!("Total sum is {}", total_sum);
    Ok(())
}


fn walk(matrix: &[Vec<char>], start:(usize,usize), visited: & mut HashSet<(usize,usize)>) -> (isize, isize) {
    
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let char = matrix[start.0][start.1];
    
    let mut to_action : Vec<(usize,usize)> = Vec::new();
    to_action.push(start);
    
    let mut perimeter = 0;
    let mut area = 0;
    
    while to_action.len() > 0 {
        if let Some(p) = to_action.pop() {
            visited.insert(p);
            let like_neighbours = directions.iter()
                .filter_map(|v| add_vector_to_positions(p,v))
                .filter(|x| matrix.len() > x.0 && matrix[x.0].len() > x.1 && matrix[x.0][x.1] == char)
                .collect::<Vec<_>>();

            perimeter += 4 - like_neighbours.len() as isize;
            area += 1;
            let to_add = like_neighbours.iter().filter(|x| !visited.contains(x) && !to_action.contains(x)).collect::<Vec<_>>();
            to_action.extend(to_add);
        }
    }

    (area,perimeter)
    
}