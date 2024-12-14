use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use rayon::prelude::*;

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    if let Some(parent_dir) = Path::new("output/example.txt").parent() {
        fs::create_dir_all(parent_dir)?; // Create the directory if it doesn't exist
    }
    
    let x_lim  = 101isize;
    let y_lim  = 103isize;

    let h = input_data.lines()
        .filter_map(|line| parse_line(line)).collect::<Vec<_>>();

    (3145..10000)
        .into_par_iter()
        .try_for_each(|i| {
            let a = h.iter()
                .map(|q| project_robot(q, i, x_lim, y_lim))
                .collect::<Vec<(usize, usize)>>();

            let name = format!("output/{}.bmp", i);

            create_bmp(&name, x_lim as usize, y_lim as usize, a)
        })?;
    Ok(())
}



fn project_robot(input: &((usize, usize), (isize, isize)), seconds: isize, x_lim : isize, y_lim :isize) -> (usize, usize) {
    
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

fn create_bmp(filename: &str, width: usize, height: usize, active_pixels: Vec<(usize, usize)>) -> io::Result<()> {
    // File header (14 bytes)
    let mut bmp_header = vec![
        0x42, 0x4D,                     // Signature "BM"
        0, 0, 0, 0,                     // File size (placeholder)
        0, 0, 0, 0,                     // Reserved
        54, 0, 0, 0,                    // Offset to pixel data
    ];

    // DIB header (40 bytes)
    let mut dib_header = vec![
        40, 0, 0, 0,                    // DIB header size
        0, 0, 0, 0,                     // Image width (placeholder)
        0, 0, 0, 0,                     // Image height (placeholder)
        1, 0,                           // Number of color planes
        24, 0,                          // Bits per pixel (24 for RGB)
        0, 0, 0, 0,                     // Compression (0 = no compression)
        0, 0, 0, 0,                     // Image size (0 for uncompressed)
        0x13, 0x0B, 0, 0,               // Horizontal resolution (72 DPI)
        0x13, 0x0B, 0, 0,               // Vertical resolution (72 DPI)
        0, 0, 0, 0,                     // Number of colors in palette
        0, 0, 0, 0,                     // Important colors
    ];

    // Update width and height in the DIB header
    dib_header[4..8].copy_from_slice(&(width as u32).to_le_bytes());
    dib_header[8..12].copy_from_slice(&(height as u32).to_le_bytes());

    // Generate pixel data
    let mut pixel_data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            // Check if this pixel is active
            if active_pixels.contains(&(x, y)) {
                pixel_data.extend_from_slice(&[255, 255, 255]); // White
            } else {
                pixel_data.extend_from_slice(&[0, 0, 0]); // Black
            }
        }
        // Padding to align to 4-byte boundary
        while pixel_data.len() % 4 != 0 {
            pixel_data.push(0);
        }
    }

    // Calculate the file size and update the header
    let file_size = bmp_header.len() + dib_header.len() + pixel_data.len();
    bmp_header[2..6].copy_from_slice(&(file_size as u32).to_le_bytes());

    // Write the BMP file
    let mut file = File::create(filename)?;
    file.write_all(&bmp_header)?;
    file.write_all(&dib_header)?;
    file.write_all(&pixel_data)?;

    println!("BMP file '{}' created successfully!", filename);
    Ok(())
}