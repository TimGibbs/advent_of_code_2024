use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let lines = input_data.lines().collect::<Vec<&str>>();

    let towels = lines[0].split(',').map(|x| x.trim()).collect::<Vec<&str>>();

    let patterns = &lines[2..];
    
    let count = patterns.iter().map(|x| is_pattern_possible(x,&towels)).filter(|x| *x)
        .count();
    println!("{}/{}", count, patterns.len());
    Ok(())
}


fn is_pattern_possible(pattern: &str, towels: &[&str]) -> bool {
    if pattern.len() == 0 { return true; }

    for towel in towels {
        if pattern.starts_with(towel) {
            if is_pattern_possible(&pattern[towel.len()..], towels) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = is_pattern_possible("bwurrg", &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(result, true);
    }
}