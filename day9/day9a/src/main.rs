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

    while let (Some(opening), Some(fileblock)) = (
        find_opening(&array_of_arrays),
        find_fileblock(&array_of_arrays),
    ) {
        if opening.0 > fileblock.0 || (opening.0 == fileblock.0 && opening.1 >= fileblock.1) {
            break;
        }

        array_of_arrays[opening.0][opening.1] = fileblock.2;
        array_of_arrays[fileblock.0][fileblock.1] = 0
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

fn find_opening(data: &[Vec<isize>]) -> Option<(usize, usize)> {
    for (i, row) in data.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value < 0 {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_fileblock(data: &[Vec<isize>]) -> Option<(usize, usize, isize)> {
    for (i, row) in data.iter().enumerate().rev() {
        for (j, &value) in row.iter().enumerate().rev() {
            if value > 0 {
                return Some((i, j, value));
            }
        }
    }
    None
}
