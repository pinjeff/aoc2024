#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Read,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut char_pos: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, l) in content.lines().enumerate() {
        for (j, b) in l.chars().enumerate() {
            if b != '.' {
                char_pos
                    .entry(b)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }

    let max_height = content.lines().count() as isize;
    let max_width = content.lines().next().unwrap().chars().count() as isize;
    let mut anti_pos: Vec<(isize, isize)> = vec![];
    for (_, v) in char_pos.into_iter() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                let lhs = v[i];
                let rhs = v[j];
                let diff = (rhs.0 - lhs.0, rhs.1 - lhs.1);
                let results: Vec<_> = (-max_height * 2..max_height * 2)
                    .into_iter()
                    .map(|c| (lhs.0 + c * diff.0, lhs.1 + c * diff.1))
                    .collect();
                anti_pos.extend(results);
            }
        }
    }

    let anti_pos: HashSet<_> = anti_pos
        .into_iter()
        .filter(|pos| {
            if pos.0 < 0 || pos.0 >= max_width {
                return false;
            }
            if pos.1 < 0 || pos.1 >= max_height {
                return false;
            }
            true
        })
        .collect();

    println!("Number of Anti-Position: {}", anti_pos.len());

    Ok(())
}
