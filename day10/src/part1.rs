use std::{collections::HashSet, error::Error, fs::File, io::Read};

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
                .map(|y| {
                    get_locations(&input, (x as isize, y as isize), 0)
                        .unwrap_or_default()
                        .len()
                })
                .sum::<usize>()
        })
        .sum();

    println!("Score: {score:?}");
    Ok(())
}

fn get_locations(
    map: &Vec<Vec<usize>>,
    pos: (isize, isize),
    target: usize,
) -> Option<HashSet<(usize, usize)>> {
    let out_of_bound = is_out_of_bound_fn(map.len() - 1);
    let pos = out_of_bound(pos)?;
    let current = map[pos.0][pos.1];

    if current != target {
        return None;
    }

    let mut positions = HashSet::new();
    if current == 9 {
        positions.insert(pos);
        return Some(positions);
    }

    get_locations(map, (pos.0 as isize - 1, pos.1 as isize - 0), target + 1)
        .unwrap_or_default()
        .into_iter()
        .for_each(|s| {
            positions.insert(s);
        });
    get_locations(map, (pos.0 as isize + 1, pos.1 as isize - 0), target + 1)
        .unwrap_or_default()
        .into_iter()
        .for_each(|s| {
            positions.insert(s);
        });
    get_locations(map, (pos.0 as isize - 0, pos.1 as isize - 1), target + 1)
        .unwrap_or_default()
        .into_iter()
        .for_each(|s| {
            positions.insert(s);
        });
    get_locations(map, (pos.0 as isize - 0, pos.1 as isize + 1), target + 1)
        .unwrap_or_default()
        .into_iter()
        .for_each(|s| {
            positions.insert(s);
        });

    Some(positions)
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
