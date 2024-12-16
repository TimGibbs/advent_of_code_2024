use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fs, io};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct State {
    x: usize,
    y: usize,
    direction: usize, // 0: East, 1: South, 2: West, 3: North
}

#[derive(Debug)]
struct Node {
    cost: usize,
    state: State,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Direction offsets: [East, South, West, North]
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_map(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in input.trim().lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        for (j, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start = (i, j);
            } else if ch == 'E' {
                end = (i, j);
            }
        }
        map.push(row);
    }

    (map, start, end)
}

fn find_tiles_in_best_paths(input: &str) -> HashSet<(usize, usize)> {
    let (map, start, end) = parse_map(input);
    let rows = map.len();
    let cols = map[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut parents: HashMap<State, Vec<State>> = HashMap::new(); // Track paths
    let mut visited_tiles = HashSet::new();

    let start_state = State { x: start.0, y: start.1, direction: 0 }; // Start facing East
    priority_queue.push(Node { cost: 0, state: start_state.clone() });
    costs.insert(start_state.clone(), 0);

    while let Some(Node { cost, state }) = priority_queue.pop() {
        if cost > *costs.get(&state).unwrap_or(&usize::MAX) {
            continue;
        }

        // Add current tile to visited tiles
        visited_tiles.insert((state.x, state.y));

        // Try moving forward
        let (dx, dy) = DIRECTIONS[state.direction];
        let nx = state.x as isize + dx;
        let ny = state.y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols && map[nx as usize][ny as usize] != '#' {
            let next_state = State { x: nx as usize, y: ny as usize, direction: state.direction };
            let next_cost = cost + 1;

            if next_cost < *costs.get(&next_state).unwrap_or(&usize::MAX) {
                costs.insert(next_state.clone(), next_cost);
                parents.insert(next_state.clone(), vec![state.clone()]);
                priority_queue.push(Node { cost: next_cost, state: next_state });
            } else if next_cost == *costs.get(&next_state).unwrap_or(&usize::MAX) {
                parents.entry(next_state.clone()).or_default().push(state.clone());
            }
        }

        // Try rotating (left and right)
        for &rotation in [-1, 1].iter() {
            let next_direction = (state.direction as isize + rotation + 4) % 4;
            let next_state = State { x: state.x, y: state.y, direction: next_direction as usize };
            let next_cost = cost + 1000;

            if next_cost < *costs.get(&next_state).unwrap_or(&usize::MAX) {
                costs.insert(next_state.clone(), next_cost);
                parents.insert(next_state.clone(), vec![state.clone()]);
                priority_queue.push(Node { cost: next_cost, state: next_state });
            } else if next_cost == *costs.get(&next_state).unwrap_or(&usize::MAX) {
                parents.entry(next_state.clone()).or_default().push(state.clone());
            }
        }
    }

    // Backtrack to find all tiles in shortest paths
    let mut stack = vec![State { x: end.0, y: end.1, direction: 0 }];
    let mut path_tiles = HashSet::new();

    while let Some(current) = stack.pop() {
        path_tiles.insert((current.x, current.y));
        if let Some(prev_states) = parents.get(&current) {
            for prev in prev_states {
                stack.push(prev.clone());
            }
        }
    }

    path_tiles
}

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let path_tiles = find_tiles_in_best_paths(&input_data);
    println!("Number of tiles part of at least one shortest path: {}", path_tiles.len());
    Ok(())
}
