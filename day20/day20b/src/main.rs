use std::{io};
use std::collections::{BinaryHeap};
use common::{add_vector_to_positions_with_bounds, load_as_2d_char_matrix, DIRECTIONS};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = load_as_2d_char_matrix(file_path)?;

    let mut end = (0usize,0usize);
    for y in 0..input_data.len() {
        for x in 0..input_data[y].len() {
            if input_data[y][x] == 'E' { end = (y,x)}
        }
    }

    let distances = distances_to_end(&input_data, end);

    let shortcut_reach = 20;
    let minimum_saved_steps = 100;
    let shortcuts = find_shortcuts(&input_data, &distances, shortcut_reach, minimum_saved_steps);
    // let c = shortcuts.into_iter().fold(HashMap::new(), |mut acc, shortcut| {
    //     *acc.entry(shortcut.saved_steps).or_insert(0) += 1;
    //     acc
    // });
    
    
    // println!("Potential shortcuts:");
    // for shortcut in shortcuts {
    //     println!(
    //         "From {:?} to {:?} via {:?} saves {} steps",
    //         shortcut.from, shortcut.to, shortcut.via, shortcut.saved_steps
    //     );
    // }

    println!("{:?}", shortcuts);
    
    Ok(())
}

fn positions_within_reach(
    position: (usize, usize),
    reach: usize,
    y_bound: usize,
    x_bound: usize,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for dy in 0..=reach {
        let dx_limit = reach - dy;

        // Check all positions within the Manhattan distance limit
        for dx in 0..=dx_limit {
            if let Some(new_y) = position.0.checked_sub(dy) {
                if let Some(new_x) = position.1.checked_sub(dx) {
                    positions.push((new_y, new_x)); 
                }
                if position.1 + dx < x_bound {
                    positions.push((new_y, position.1 + dx));
                }
            }
            if position.0 + dy < y_bound {
                if let Some(new_x) = position.1.checked_sub(dx) {
                    positions.push((position.0 + dy, new_x));
                }
                if position.1 + dx < x_bound {
                    positions.push((position.0 + dy, position.1 + dx));
                }
            }
        }
    }

    positions.sort();
    positions.dedup();

    positions
}

fn manhattan_distance(point1: (usize, usize), point2: (usize, usize)) -> usize {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    x1.max(x2) - x1.min(x2) + y1.max(y2) - y1.min(y2)
}

pub fn distances_to_end(map: &Vec<Vec<char>>, end: (usize, usize)) -> Vec<Vec<Option<usize>>> {
    let rows = map.len();
    let cols = map[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut costs = vec![vec![None; cols]; rows]; // Initialize distances to None

    priority_queue.push((0i32, end));
    costs[end.0][end.1] = Some(0); 

    while let Some((neg_cost, state)) = priority_queue.pop() {
        let cost = -neg_cost as usize;

        if let Some(existing_cost) = costs[state.0][state.1] {
            if cost > existing_cost {
                continue;
            }
        }

        let neighbors: Vec<(usize, usize)> = DIRECTIONS.iter()
            .filter_map(|d| add_vector_to_positions_with_bounds(state, d, rows, cols))
            .filter(|p| map[p.0][p.1] != '#')
            .collect();

        let next_cost = cost + 1;
        for neighbor in neighbors {
            if costs[neighbor.0][neighbor.1].map_or(true, |existing_cost| next_cost < existing_cost) {
                costs[neighbor.0][neighbor.1] = Some(next_cost);
                priority_queue.push((-(next_cost as i32), neighbor));
            }
        }
    }

    costs
}

fn find_shortcuts(
    map: &Vec<Vec<char>>,
    distances: &Vec<Vec<Option<usize>>>,
    reach: usize,
    minimum_saved_steps: usize,
) -> usize {
    let mut shortcuts = 0usize;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Some(current_distance) = distances[y][x] {
                let neighbors = positions_within_reach((y, x), reach, map.len(), map[0].len());
                for neighbor in neighbors {
                    if let Some(neighbor_distance) = distances[neighbor.0][neighbor.1] {
                        let traverse_cost = manhattan_distance((y, x), neighbor);
                        let total_cost_via_neighbor = neighbor_distance + traverse_cost;
                        if total_cost_via_neighbor < current_distance {
                            let saved_steps = current_distance - total_cost_via_neighbor;
                            if saved_steps >= minimum_saved_steps {
                                shortcuts +=1
                            }
                        }
                    }
                }
            }
        }
    }

    shortcuts
}