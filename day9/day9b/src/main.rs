use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let array = input_data
        .chars()
        .filter_map(|c| c.to_digit(10).map(|x| x as isize))
        .collect::<Vec<isize>>();

    let mut array_of_arrays: Vec<Vec<isize>> = array
        .iter()
        .enumerate()
        .map(|(index, &x)| {
            if index % 2 == 0 {
                vec![(index / 2) as isize; x as usize]
            } else {
                vec![-1; x as usize]
            }
        })
        .collect();

    for i in (0..array_of_arrays.len()).rev() {
        if array_of_arrays[i].len() == 0 || array_of_arrays[i][0] <0 {
            continue;
        }
        if let Some(opening) = find_opening(&array_of_arrays,array_of_arrays[i].len()) {
            if opening >= i { continue;}
            let mut x = 0usize;
            for j in 0..array_of_arrays[opening].len() {
                if x == array_of_arrays[i].len() {
                    break;
                }
                if array_of_arrays[opening][j] <0 { 
                    array_of_arrays[opening][j] = array_of_arrays[i][x];
                    array_of_arrays[i][x] = -1;
                    x += 1;
                }
            }
        }
    }
    
    let mut position = 0;
    let mut val = 0;
    for x in array_of_arrays {
        for y in x {
            if y > 0 {
                val += y * position;
            }
            position += 1;
        }
    }
    println!("{}", val);

    Ok(())
}

fn find_opening(data: &[Vec<isize>], size:usize) -> Option<usize> {
    for (i, row) in data.iter().enumerate() {
        if row.iter().filter(|&&x| x < 0).count() >= size {
            return Some(i);
        }
    }
    None
}
