use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut input: Vec<usize> = content
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();

    const ITERATIONS: usize = 25;
    let mut l = 0;
    while l < ITERATIONS {
        let mut results: Vec<usize> = vec![];
        for n in &input {
            if *n == 0 {
                results.push(1);
                continue;
            }
            let num_digit = n.ilog10() + 1;
            if num_digit % 2 == 0 {
                let lhs = n / (10usize.pow(num_digit / 2));
                let rhs = n - lhs * 10usize.pow(num_digit / 2);
                results.push(lhs);
                results.push(rhs);
                continue;
            }
            results.push(n * 2024);
        }
        input = results;
        l += 1;
    }

    println!("Number of stones: {}", input.len());
    Ok(())
}
