use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let lines = input_data.lines().collect::<Vec<&str>>();

    let towels = lines[0].split(',').map(|x| x.trim()).collect::<Vec<&str>>();

    let patterns = &lines[2..];

    let mut memo = HashMap::new();
    let count: usize = patterns
        .iter()
        .map(|x| pattern_count(x, &towels, &mut memo))
        .sum();

    println!("{}", count);
    Ok(())
}

fn pattern_count<'a>(pattern: &'a str, towels: &[&str], memo: &mut HashMap<&'a str, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(&cached) = memo.get(pattern) {
        return cached;
    }

    let mut count = 0;

    for towel in towels {
        if pattern.starts_with(towel) {
            let t = pattern_count(&pattern[towel.len()..], towels, memo);
            count += t;
        }
    }

    memo.insert(pattern, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut memo = HashMap::new();
        let result = pattern_count("gbbr", &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &mut memo);
        assert_eq!(result, 4);
    }

    #[test]
    fn case2() {
        let mut memo = HashMap::new();
        let result = pattern_count("rrbgbr", &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &mut memo);
        assert_eq!(result, 6);
    }

    #[test]
    fn case3() {
        let mut memo = HashMap::new();
        let result = pattern_count("bwurrg", &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &mut memo);
        assert_eq!(result, 1);
    }

    #[test]
    fn case4() {
        let mut memo = HashMap::new();
        let result = pattern_count("brwrr", &["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &mut memo);
        assert_eq!(result, 2);
    }
}
