use std::{fs, io};
use std::collections::{BinaryHeap, HashMap};
use common::add_vector_to_positions_with_bounds;


fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let coords: Vec<(i32,i32)> = input_data
        .lines()
        .map(|l| {
            let parts: Vec<i32> = l.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            (parts[0], parts[1])
        }).collect();
    
    for i in  (0..coords.len()).rev(){
        let map = create_map(&coords,(71,71),i);
        let sp = shortest_path(&map,(0,0),(70,70));
        println!("{},{} :{}", coords[i].0, coords[i].1, sp.is_some());
        if sp.is_some() {
            print_map(&map);
            break;
        }
    }
    Ok(())
}


fn create_map(coords: &[(i32,i32)], size:(usize,usize), bytes_count:usize) -> Vec<Vec<bool>> {

    let mut map = vec![vec![true; size.0]; size.1];
    for i in 0..bytes_count-1{
        let byte = coords[i];
        map[byte.1 as usize][byte.0 as usize] = false;
    }
    map
}

fn print_map(map: &Vec<Vec<bool>>) {
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            print!("{}", if map[i][j] { '.'} else { '#'});
        }
        print!("\n");
    }
}
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
fn shortest_path(map: &Vec<Vec<bool>>, start:(usize,usize), end:(usize,usize)) -> Option<usize> {

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
            .filter(|p| map[p.0][p.1])
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