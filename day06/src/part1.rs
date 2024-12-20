use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut maze: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let mut pos = (0, 0);
    let mut direction: (isize, isize) = (-1, 0);
    'outer: for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == '^' {
                pos = (i, j);
                break 'outer;
            }
        }
    }

    loop {
        let x = pos.0.checked_add_signed(direction.0);
        let y = pos.1.checked_add_signed(direction.1);
        let (x, y) = match (x, y) {
            (Some(x), Some(y)) => (x, y),
            _ => break,
        };

        if x >= maze.len() {
            break;
        }
        if y >= maze[0].len() {
            break;
        }

        let next_block = &maze[x][y];
        match next_block {
            '#' => {
                direction = get_next_direction(direction);
                continue;
            }
            _ => {
                maze[pos.0][pos.1] = 'x';
                pos = (x, y);
            }
        }
    }

    let distinct_pos_count: usize = maze
        .iter()
        .map(|r| r.iter().filter(|c| **c == 'x').count())
        .sum();

    println!("Distinct positions: {}", distinct_pos_count + 1);
    Ok(())
}

fn get_next_direction((x, y): (isize, isize)) -> (isize, isize) {
    match (x, y) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => unreachable!(),
    }
}
