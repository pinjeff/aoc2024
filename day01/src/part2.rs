use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut left_numbers: Vec<usize> = vec![];
    let mut right_numbers: Vec<usize> = vec![];

    for line in content.lines() {
        let split = line.split_once("   ").unwrap();
        left_numbers.push(split.0.parse().unwrap());
        right_numbers.push(split.1.parse().unwrap());
    }

    let score: usize = left_numbers
        .iter()
        .map(|lhs| lhs * right_numbers.iter().filter(|rhs| lhs == *rhs).count())
        .sum();

    println!("Score: {score}");

    Ok(())
}
