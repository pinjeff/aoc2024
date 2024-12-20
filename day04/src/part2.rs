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
    for i in 1..(input.len() - 1) {
        for j in 1..(input[i].len() - 1) {
            if does_match(&input, (i, j)) {
                matches += 1;
            }
        }
    }

    println!("Number of matches: {matches}");
    Ok(())
}

fn does_match(arr: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    let char = arr[position.0][position.1];
    if char != 'A' {
        return false;
    }

    let mut neighbors = ['A'; 4];
    neighbors[0] = arr[position.0 - 1][position.1 - 1];
    neighbors[1] = arr[position.0 + 1][position.1 - 1];
    neighbors[2] = arr[position.0 - 1][position.1 + 1];
    neighbors[3] = arr[position.0 + 1][position.1 + 1];

    if neighbors.iter().filter(|c| **c == 'S').count() != 2 {
        return false;
    };
    if neighbors.iter().filter(|c| **c == 'M').count() != 2 {
        return false;
    };
    if neighbors[0] == neighbors[3] {
        return false;
    }

    true
}
