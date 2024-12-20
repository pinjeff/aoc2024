use memoize::memoize;
use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let input: Vec<usize> = content
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    const ITERATIONS: usize = 75;
    let sum: usize = input
        .into_iter()
        .map(|n| len_recursive(n, ITERATIONS))
        .sum();

    println!("Number of stones: {}", sum);

    Ok(())
}

#[memoize]
fn len_recursive(number: usize, iteration: usize) -> usize {
    if iteration == 0 {
        return 1;
    }
    if number == 0 {
        return len_recursive(1, iteration - 1);
    }
    let num_digit = number.ilog10() + 1;
    if num_digit % 2 == 0 {
        let lhs = number / (10usize.pow(num_digit / 2));
        let rhs = number - lhs * 10usize.pow(num_digit / 2);
        let lhs_len = len_recursive(lhs, iteration - 1);
        let rhs_len = len_recursive(rhs, iteration - 1);
        return lhs_len + rhs_len;
    }

    len_recursive(number * 2024, iteration - 1)
}
