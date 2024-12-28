use std::{io};
use common::load_as_2d_char_matrix;
use common::shortest_path;
use rayon::prelude::*;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = load_as_2d_char_matrix(file_path)?;

    let mut start = (0usize,0usize);
    let mut end = (0usize,0usize);
    for y in 0..input_data.len() {
        for x in 0..input_data[y].len() {
            if input_data[y][x] == 'S' { start = (y,x)}
            if input_data[y][x] == 'E' { end = (y,x)}
        }
    }

    let baseline = shortest_path(&input_data, start, end).unwrap();
    println!("{}", baseline);

    let walls: Vec<(usize, usize)> = input_data
        .iter().enumerate()
        .flat_map(|(y, row)|
            row.iter().enumerate()
                .filter_map(|(x, v)|
                    if *v == '#'
                    { Some((y,x)) }
                    else { None }
                ).collect::<Vec<(usize, usize)>>())
        .collect::<Vec<(usize, usize)>>();

    println!("{}", walls.len());
    
    let lower_limit = 100usize;

    let h = walls.par_iter()
        .filter_map(|(y,x)|{
            let mut m = input_data.clone();
            m[*y][*x] = '.';
            let a = shortest_path(&m,start, end)?;
            if baseline - a >= lower_limit {
                Some(baseline - a)
            } else {
                None
            }
        }).collect::<Vec<usize>>();

    let count = h.len();

    println!("{:?}", h);
    println!("{:?}", count);


    Ok(())
}

// fn edit_maze(original : &[Vec<char>], edit_point : (usize, usize)) -> Vec<Vec<char>> {
//     //not wall wont matter
//     if original[edit_point.0][edit_point.1] != '#' { return Vec::new(); }
// 
//     let neighbours : Vec<(usize,usize)> = DIRECTIONS.iter()
//         .filter_map(|z| add_vector_to_positions_with_bounds(edit_point,z,original.len(), original[0].len()))
//         .filter(|z| original[z.0][z.1] != '#')
//         .collect();
// 
//     //wall stands alone
//     if neighbours.is_empty() {
//         return Vec::new();
//     }
// 
//     let mut clone2 = original.to_vec();
//     clone2[edit_point.0][edit_point.1] = '1';
//     clone2
// }
