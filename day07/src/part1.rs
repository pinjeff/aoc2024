use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let input: Vec<(usize, Vec<usize>)> = content
        .lines()
        .map(|l| {
            let (target, numbers) = l.split_once(": ").unwrap();
            let target: usize = target.parse().unwrap();
            let numbers: Vec<usize> = numbers
                .split(" ")
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect();

            (target, numbers)
        })
        .collect();

    let passing_sum: usize = input
        .into_iter()
        .filter(|(t, n)| {
            generate_combinations(n.len() - 1)
                .into_iter()
                .map(|ops| get_result(n, &ops))
                .any(|res| res == *t)
        })
        .map(|(t, _)| t)
        .sum();

    println!("Passing sum: {passing_sum}");
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Plus,
    Mult,
}

fn get_result(numbers: &[usize], operators: &[Operator]) -> usize {
    numbers
        .iter()
        .skip(1)
        .zip(operators.into_iter())
        .fold(numbers[0], |acc, (rhs, op)| eval(acc, *rhs, *op))
}

fn eval(lhs: usize, rhs: usize, op: Operator) -> usize {
    match op {
        Operator::Plus => lhs + rhs,
        Operator::Mult => lhs * rhs,
    }
}

fn generate_combinations(length: usize) -> Vec<Vec<Operator>> {
    if length == 0 {
        return vec![vec![]];
    }

    let smaller_combinations = generate_combinations(length - 1);
    let mut result = Vec::new();

    for combo in smaller_combinations {
        result.push([combo.clone(), vec![Operator::Plus]].concat());
        result.push([combo.clone(), vec![Operator::Mult]].concat());
    }

    result
}
