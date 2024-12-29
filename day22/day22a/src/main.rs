use std::{fs, io};

fn main() -> io::Result<()> {
    let file_path = "../data.txt";
    let input_data = fs::read_to_string(file_path)?;

    let s: Vec<usize>  = input_data.lines()
        .into_iter()
        .flat_map(|x| x.parse::<usize>()).collect();
    
    let mut sum = 0;
    for secret in s {
        let g = advance(secret,2000);
        println!("{} : {}", secret, g);
        sum += g;
    }
    println!("{}", sum);
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

fn advance(secret: usize, count: usize) -> usize {
    let mut r = secret;
    for _ in 0..count {
        r = secret_step(r);
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = mix(42, 15);
        assert_eq!(result,37);
    }

    #[test]
    fn case2() {
        let result = prune(100000000);
        assert_eq!(result,16113920);
    }

    #[test]
    fn case3() {
        let mut result = secret_step(123);
        assert_eq!(result,15887950);
        result = secret_step(result);
        assert_eq!(result,16495136);
        result = secret_step(result);
        assert_eq!(result,527345);
        result = secret_step(result);
        assert_eq!(result,704524);
        result = secret_step(result);
        assert_eq!(result,1553684);
        result = secret_step(result);
        assert_eq!(result,12683156);
        result = secret_step(result);
        assert_eq!(result,11100544);
        result = secret_step(result);
        assert_eq!(result,12249484);
        result = secret_step(result);
        assert_eq!(result,7753432);
        result = secret_step(result);
        assert_eq!(result,5908254);
    }
}