use std::{error::Error, fs::File, io::Read, iter::zip};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut left_numbers: Vec<isize> = vec![];
    let mut right_numbers: Vec<isize> = vec![];

    for line in content.lines() {
        let split = line.split_once("   ").unwrap();
        left_numbers.push(split.0.parse().unwrap());
        right_numbers.push(split.1.parse().unwrap());
    }

    left_numbers.sort();
    right_numbers.sort();
    let mut diff = 0;
    for (left, right) in zip(left_numbers.iter(), right_numbers.iter()) {
        diff += (left - right).abs();
    }

    println!("Diff: {diff}");

    Ok(())
}
