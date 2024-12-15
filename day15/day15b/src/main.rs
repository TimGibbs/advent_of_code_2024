use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let parts = input_data.split("\r\n\r\n").collect::<Vec<&str>>();
    let grid_data = parts[0];
    let commands_data = parts[1];

    let mut grid: Vec<Vec<char>> = grid_data
        .lines()
        .map(|line| line
            .chars()
            .map(|c| {
                match c {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => ['X','X']
                }
            })
            .flat_map(|x| x).collect())
        .collect();

    let commands: Vec<char> = commands_data
        .chars()
        .filter(|&c| c != '\n') // Remove newline characters
        .collect();

    let mut robot = (0,0);

    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' { 
                robot = (i,j);
                break 'outer;
            }
            
        }
    }
    // for g in grid.iter_mut() {
    //     println!("{:?}", g);
    // }
    for ch in commands {
        println!("{}", ch);
        (grid,robot) =  match ch {
            '^' => move_robot(&grid,robot,(-1,0)),
            'v'  => move_robot(&grid,robot,(1,0)),
            '<'  => move_robot(&grid,robot,(0,-1)),
            '>'  => move_robot(&grid,robot,(0,1)),
            _ => (grid.clone(),robot)
        };
        // for g in grid.iter_mut() {
        //     println!("{:?}", g);
        // }
        // println!("");
    }

    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '[' {
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
    
    g[y][x] = '.';
    
    let new_x = (x as isize + dx) as usize;
    let new_y = (y as isize + dy) as usize;
    if g[new_y][new_x] == '.' {
        g[new_y][new_x] = '@';
        return (g,(new_y, new_x));
    }
    
    if direction.0 == 0 {
        g = move_box_horizontally(&g,(new_y,new_x), direction);
        g[new_y][new_x] = '@';
        return (g,(new_y, new_x));
    }
    
    if g[new_y][new_x] == '[' {
        g = move_box_vertically(&g, (new_y,new_x), direction);
        g[new_y][new_x] = '@';        
        return (g,(new_y, new_x));
    }

    if g[new_y][new_x] == ']' {
        g = move_box_vertically(&g, (new_y,new_x-1), direction);
        g[new_y][new_x] = '@';
        return (g,(new_y, new_x));
    }
    (g,(new_y, new_x))

}

fn move_box_horizontally(grid: &Vec<Vec<char>>, position: (usize, usize), direction: (isize, isize)) -> Vec<Vec<char>> {

    if grid[position.0][position.1] == ']' { return move_box_horizontally(grid, (position.0, position.1-1), direction); }
    //assume position is of [
    let mut g = grid.clone();
    let (_, dx) = direction;
    let (y, x) = position;

    if dx == -1 {
        if g[y][x-2] == '[' {
            g = move_box_horizontally(&g, (y,x-2), direction);
        }
        g[y][x-1] = '[';
        g[y][x] = ']';
    }
    if dx == 1 {
        if g[y][x+2] == '[' {
            g = move_box_horizontally(&g, (y,x+2), direction);
        }
        g[y][x+1] = '[';
        g[y][x+2] = ']';
    }
    g
}

fn move_box_vertically(grid: &Vec<Vec<char>>, position: (usize, usize), direction: (isize, isize)) -> Vec<Vec<char>> {

    if grid[position.0][position.1] == ']' { return move_box_vertically(grid, (position.0, position.1-1), direction); }
    //assume position is of [
    let mut g = grid.clone();
    let (dy, _) = direction;
    let (y, x) = position;
    
    let new_y = (y as isize + dy) as usize;
    
    if g[new_y][x] == ']' {
        g = move_box_vertically(&g, (new_y, x-1), direction);
    }

    if g[new_y][x] == '[' {
        g = move_box_vertically(&g, (new_y, x), direction);
    }
    
    if g[new_y][x+1] == '[' {
        g = move_box_vertically(&g, (new_y, x+1), direction);
    }
    
    g[new_y][x] = '[';
    g[new_y][x+1] = ']';
    g[y][x] = '.';
    g[y][x+1] = '.';
    g
}

fn is_movable(grid: &Vec<Vec<char>>, position: (usize, usize), direction: (isize, isize)) -> bool {
    let (dy, dx) = direction;
    let (y, x) = position;

    let new_x = (x as isize + dx) as usize;
    let new_y = (y as isize + dy) as usize;

    if grid[new_y][new_x] == '.' { return true; }
    if grid[new_y][new_x] == '#' { return false; }
    
    if direction.0 == 0 {
        return is_movable(grid, (new_y, new_x), direction);
    }
    
    if grid[new_y][new_x] == '[' {
        return is_movable(grid, (new_y, new_x), direction) 
            &&  is_movable(grid, (new_y, new_x+1), direction);
    }
    if grid[new_y][new_x] == ']' {
        return is_movable(grid, (new_y, new_x), direction)
            &&  is_movable(grid, (new_y, new_x-1), direction);
    }
    
    is_movable(grid, (new_y, new_x), direction)
        &&  is_movable(grid, (new_y, new_x-1), direction)
    
}