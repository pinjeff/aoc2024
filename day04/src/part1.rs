use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let input: Vec<Vec<char>> = content
        .lines()
        .into_iter()
        .map(|l| l.chars().collect())
        .collect();

    let mut matches = 0;
    for i in 0..(input.len()) {
        for j in 0..(input[i].len()) {
            matches += no_matches(&input, (i, j), (0, 0), 0)
        }
    }

    println!("Number of matches: {matches}");
    Ok(())
}

fn no_matches(
    arr: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: (isize, isize),
    depth: usize,
) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 1),
        (1, 0),
    ];

    let x: isize = position.0 as isize + direction.0;
    let y: isize = position.1 as isize + direction.1;
    if x < 0 || x >= arr.len() as isize {
        return 0;
    }
    if y < 0 || y >= arr.len() as isize {
        return 0;
    }

    let char = arr[x as usize][y as usize];
    match (char, depth) {
        ('X', 0) => directions
            .into_iter()
            .map(|d| no_matches(&arr, position, d, 1))
            .filter(|m| *m == 4)
            .count(),
        ('M', 1) => no_matches(&arr, (x as usize, y as usize), direction, 2),
        ('A', 2) => no_matches(&arr, (x as usize, y as usize), direction, 3),
        ('S', 3) => 4,
        _ => 0,
    }
}
