use std::{io};
use std::collections::HashSet;
use common::load_as_2d_char_matrix;
use common::add_vector_to_positions_with_bounds;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let matrix = load_as_2d_char_matrix(file_path)?;

    let mut total_sum = 0;

    let mut visited : HashSet<(usize,usize)> = HashSet::new();

    for i  in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if !visited.contains(&(i,j)) {
                let (area,perimeter, edges) = walk(&matrix,(i,j),&mut visited);
                total_sum += area*edges;
                println!("{} a:{}, p:{}, e:{}", matrix[i][j] , area,perimeter, edges);
            }
        }
    }
    println!("Total sum is {}", total_sum);
    Ok(())
}


fn walk(matrix: &[Vec<char>], start:(usize,usize), visited: & mut HashSet<(usize,usize)>) -> (usize, usize, usize) {

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let char = matrix[start.0][start.1];

    let mut to_action : Vec<(usize,usize)> = Vec::new();
    to_action.push(start);

    let mut perimeter = 0;
    let mut area = 0;
    let mut corners = 0;

    while to_action.len() > 0 {
        if let Some(p) = to_action.pop() {
            visited.insert(p);
            let like_neighbours = directions.iter()
                .filter_map(|v| add_vector_to_positions_with_bounds(p,v, matrix.len(), matrix[p.1].len()))
                .filter(|x| matrix.len() > x.0 && matrix[x.0].len() > x.1 && matrix[x.0][x.1] == char)
                .collect::<Vec<_>>();

            perimeter += 4 - like_neighbours.len();
            area += 1;
            corners += corner_count(matrix, p);

            let to_add = like_neighbours.iter().filter(|x| !visited.contains(x) && !to_action.contains(x)).collect::<Vec<_>>();
            to_action.extend(to_add);
        }
    }

    (area,perimeter, corners)

}

fn corner_count(matrix: &[Vec<char>], position:(usize,usize)) -> usize {
    let val = matrix[position.0][position.1];
    let ul = position_matches(matrix, position, &(-1,-1),val);
    let u = position_matches(matrix, position, &(-1,0),val);
    let ur = position_matches(matrix, position, &(-1,1),val);
    let l =position_matches(matrix, position, &(0,-1),val);
    let r = position_matches(matrix, position, &(0,1),val);
    let dl = position_matches(matrix, position, &(1,-1),val);
    let d = position_matches(matrix, position, &(1,0),val);
    let dr = position_matches(matrix, position, &(1,1),val);

    let mut corner = 0;

    if u ==false && r==false {
        corner += 1;
    }

    if r==false && d==false {
        corner += 1;
    }

    if d==false && l==false {
        corner += 1;
    }

    if l==false && u==false {
        corner += 1;
    }

    if u==true && r==true && ur==false {
        corner += 1;
    }

    if r==true && d==true && dr==false {
        corner += 1;
    }

    if d==true && l==true && dl==false {
        corner += 1;
    }

    if l==true && u==true && ul==false {
        corner += 1;
    }

    corner
}

fn position_matches(matrix: &[Vec<char>], position: (usize, usize), vector: &(isize,isize), val: char) -> bool {
    add_vector_to_positions_with_bounds(position, vector, matrix.len(), matrix[0].len()).map(|x| matrix[x.0][x.1] == val).unwrap_or_else(|| false)
}
