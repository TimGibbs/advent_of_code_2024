use std::{fs, io};
use std::collections::{BinaryHeap, HashMap};

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

pub fn add_vector_to_positions_with_bounds(position: (usize, usize), vector: &(isize, isize), y_bound: usize, x_bound: usize) -> Option<(usize, usize)> {
    let y = add_isize_to_usize(position.0, vector.0)?;
    let x = add_isize_to_usize(position.1, vector.1)?;
    if y>= y_bound { return None; }
    if x>= x_bound { return None; }
    Some((y, x))
}
pub const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn shortest_path(map: &Vec<Vec<char>>, start:(usize,usize), end:(usize,usize)) -> Option<usize> {

    let rows = map.len();
    let cols = map[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut costs = HashMap::new();

    priority_queue.push((0i32, start.clone()));
    costs.insert(start.clone(), 0);

    while let Some((neg_cost, state)) = priority_queue.pop() {
        let cost = -neg_cost as usize;
        if state == end {
            return Some(cost);
        }

        if cost > *costs.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }

        let next: Vec<(usize,usize)> = DIRECTIONS.iter()
            .filter_map(|d| add_vector_to_positions_with_bounds(state, d, rows, cols))
            .filter(|p| map[p.0][p.1] != '#')
            .collect();

        let next_cost = cost+1;
        for next_state in next {
            if next_cost < *costs.get(&next_state).unwrap_or(&usize::MAX) {
                costs.insert(next_state.clone(), next_cost);
                priority_queue.push((-(next_cost as i32), next_state));
            }
        }

    }
    None
}

pub fn shortest_path_with_intermediate(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    intermediate: (usize, usize),
    end: (usize, usize),
) -> Option<usize> {
    
    // First, find the path from start to intermediate
    let cost_to_intermediate = shortest_path(map, start, intermediate)?;
    // Then, find the path from intermediate to end
    let cost_to_end = shortest_path(map, intermediate, end)?;

    // Combine both costs
    Some(cost_to_intermediate + cost_to_end)
}
