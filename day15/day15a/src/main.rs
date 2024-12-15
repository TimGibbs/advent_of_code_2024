use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;
    
    let parts = input_data.split("\r\n\r\n").collect::<Vec<&str>>();
    let grid_data = parts[0];
    let commands_data = parts[1];

    let mut grid: Vec<Vec<char>> = grid_data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let commands: Vec<char> = commands_data
        .chars()
        .filter(|&c| c != '\n') // Remove newline characters
        .collect();
    
    let mut robot = (0,0);
    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' { robot = (i,j) }
        }
    }
    
    for ch in commands {
        
        (grid,robot) =  match ch {
            '^' => move_robot(&grid,robot,(-1,0)),
            'v'  => move_robot(&grid,robot,(1,0)),
            '<'  => move_robot(&grid,robot,(0,-1)),
            '>'  => move_robot(&grid,robot,(0,1)),
            _ => (grid.clone(),robot)
        };
    }
    
    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' { 
                sum += i*100 + j
            }
        }
    }
    
    println!("{:?}", sum);
    
    
    Ok(())
}

fn move_robot(
    grid: &Vec<Vec<char>>,
    robot: (usize, usize),
    direction: (isize, isize),
) -> (Vec<Vec<char>>,(usize, usize)) {
    let mut g = grid.clone();
    
    if !is_movable(grid, robot, direction) { return (g, robot); }
    
    let (dy, dx) = direction;
    let (y, x) = robot;
    
    let new_x = (x as isize + dx) as usize;
    let new_y = (y as isize + dy) as usize;
    if g[new_y][new_x] == '.' {
        g[y][x] = '.';
        g[new_y][new_x] = '@';
        return (g,(new_y, new_x));
    }
    
    let mut temp_x = x;
    let mut temp_y = y;
    while g[(temp_y as isize + dy) as usize][(temp_x as isize + dx) as usize] == 'O' {
        temp_x = (temp_x as isize + dx) as usize;
        temp_y = (temp_y as isize + dy) as usize;
    }
    
    let slide_x = (temp_x as isize + dx) as usize;
    let slide_y = (temp_y as isize + dy) as usize;
    g[slide_y][slide_x] = 'O';
    g[y][x] = '.';
    g[new_y][new_x] = '@';
    (g,(new_y, new_x))
    

}

fn is_movable(grid: &Vec<Vec<char>>, position: (usize, usize), direction: (isize, isize)) -> bool {
    let (dy, dx) = direction;
    let (y, x) = position;

    let new_x = (x as isize + dx) as usize;
    let new_y = (y as isize + dy) as usize;

    if grid[new_y][new_x] == '.' { return true; }
    if grid[new_y][new_x] == '#' { return false; }
    is_movable(grid, (new_y, new_x), direction)
}