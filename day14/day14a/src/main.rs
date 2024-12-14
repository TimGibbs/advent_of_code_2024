use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let x_lim  = 101isize;
    let y_lim  = 103isize;
    
    let h = input_data.lines()
        .filter_map(|line| parse_line(line))
        .map(|q|project_robot(q, 100, x_lim, y_lim))
        .fold((0, 0, 0, 0), |mut acc, q| {
            let region = if (q.0 as isize) < x_lim / 2 && (q.1 as isize) < y_lim / 2 {
                (1, 0, 0, 0)
            } else if (q.0 as isize) < x_lim / 2 && (q.1 as isize) > y_lim / 2 {
                (0, 1, 0, 0)
            } else if (q.0 as isize) > x_lim / 2 && (q.1 as isize) > y_lim / 2 {
                (0, 0, 1, 0)
            } else if (q.0 as isize) > x_lim / 2 && (q.1 as isize) < y_lim / 2 {
                (0, 0, 0, 1)
            } else {
                (0, 0, 0, 0)
            };
            acc.0 += region.0;
            acc.1 += region.1;
            acc.2 += region.2;
            acc.3 += region.3;
            acc
        });
    
    
    let res = h.0*h.1*h.2*h.3;
    println!("{:?}", h);
    println!("{:?}", res);
    
    
    Ok(())
}

fn project_robot(input: ((usize, usize), (isize, isize)), seconds: isize, x_lim : isize, y_lim :isize) -> (usize, usize) {


    
    let mut x = (input.0 .0 as isize + (input.1 .0 * seconds)) % x_lim;
    let mut y = (input.0 .1 as isize + (input.1 .1 * seconds)) % y_lim;

    if x < 0 { x = x_lim + x}
    if y < 0 { y = y_lim + y}
    (x as usize, y as usize)
}

fn parse_line(input: &str) -> Option<((usize, usize), (isize, isize))> {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    
    let pos_part = parts[0].trim_start_matches("p=");
    let vel_part = parts[1].trim_start_matches("v=");

    let position = parse_tuple(pos_part)?;
    let velocity = parse_tuple(vel_part)?;
    Some(((position.0 as usize, position.1 as usize),velocity))
}

fn parse_tuple(input: &str) -> Option<(isize, isize)> {
    let nums: Vec<isize> = input
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    if nums.len() == 2 {
        Some((nums[0], nums[1]))
    } else {
        None
    }
}