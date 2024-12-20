use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let input: Vec<Vec<usize>> = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| String::from(c).parse().expect(&format!("{c}")))
                .collect()
        })
        .collect();

    let score: usize = (0..input.len())
        .into_iter()
        .map(|x| {
            (0..input[0].len())
                .map(|y| get_score(&input, (x as isize, y as isize), 0).unwrap_or(0))
                .sum::<usize>()
        })
        .sum();

    println!("Score: {score}");
    Ok(())
}

fn get_score(map: &Vec<Vec<usize>>, pos: (isize, isize), target: usize) -> Option<usize> {
    let out = is_out_of_bound_fn(map.len() - 1);
    let pos = out(pos)?;
    let current = map[pos.0][pos.1];

    if current != target {
        return None;
    }

    if current == 9 {
        return Some(1);
    }

    let mut score = 0;
    score += get_score(map, (pos.0 as isize - 1, pos.1 as isize - 0), target + 1).unwrap_or(0);
    score += get_score(map, (pos.0 as isize + 1, pos.1 as isize - 0), target + 1).unwrap_or(0);
    score += get_score(map, (pos.0 as isize - 0, pos.1 as isize - 1), target + 1).unwrap_or(0);
    score += get_score(map, (pos.0 as isize - 0, pos.1 as isize + 1), target + 1).unwrap_or(0);

    Some(score)
}

fn is_out_of_bound_fn(bound: usize) -> impl Fn((isize, isize)) -> Option<(usize, usize)> {
    move |pos| {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }
        if pos.0 > bound as isize || pos.1 > bound as isize {
            return None;
        }

        Some((pos.0 as usize, pos.1 as usize))
    }
}
