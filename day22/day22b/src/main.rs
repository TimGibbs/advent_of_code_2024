use std::{fs, io};
use std::collections::{HashMap, HashSet};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let s: Vec<usize>  = input_data.lines()
        .into_iter()
        .flat_map(|x| x.parse::<usize>()).collect();

    let mut price_map = HashMap::new();

    let mut visited = HashSet::new();

    for secret in s {

        let prices = generate_prices(secret,2000);

        let diffs: Vec<isize> = prices
            .windows(2) // Create overlapping windows of size 2
            .map(|pair| pair[1] as isize - pair[0] as isize) // Compute the difference
            .collect();

        for i in 3..diffs.len() {
            let key = (diffs[i-3],diffs[i-2],diffs[i-1], diffs[i]);
            if visited.insert(key) {
                let val = prices[i+1];
                price_map.entry(key).and_modify(|value| *value += val).or_insert(val);
            }
        }
        
        visited = HashSet::new();
        
    }
    
    let max = price_map.values().max().unwrap();
    println!("{}", max);
    Ok(())
}

fn mix(secret: usize, result: usize) -> usize {
    secret ^ result
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn secret_step_1(secret: usize) -> usize {
    prune(mix(secret, secret*64))
}

fn secret_step_2(secret: usize) -> usize {
    prune(mix(secret, secret/32))
}

fn secret_step_3(secret: usize) -> usize {
    prune(mix(secret, secret*2048))
}

fn secret_step(secret: usize) -> usize {
    secret_step_3(secret_step_2(secret_step_1(secret)))
}

fn generate_prices(secret: usize, count: usize) -> Vec<usize> {
    let mut ret = Vec::with_capacity(count);
    let mut r = secret;
    for _ in 0..count {
        r = secret_step(r);
        ret.push(r%10);
    }
    ret
}
