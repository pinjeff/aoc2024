use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let original_maze: Vec<Vec<u8>> = content.lines().map(|l| l.bytes().collect()).collect();
    let mut starting_pos = (0, 0);
    'outer: for i in 0..original_maze.len() {
        for j in 0..original_maze[i].len() {
            if original_maze[i][j] == b'^' {
                starting_pos = (i, j);
                break 'outer;
            }
        }
    }
    let starting_pos = starting_pos;

    let no_blocking: usize = (0..original_maze.len())
        .into_iter()
        .map(|x| {
            (0..original_maze[0].len())
                .into_iter()
                .map(|y| match ((x, y), original_maze[x][y]) {
                    (pos, _) if pos == starting_pos => false,
                    (_, b'#') => false,
                    _ => {
                        let mut maze = original_maze.clone();
                        maze[x][y] = b'#';
                        maze_would_loop(maze, starting_pos)
                    }
                })
                .filter(|b| *b)
                .count()
        })
        .sum();

    println!("Possible looping positions: {no_blocking}");

    Ok(())
}

fn maze_would_loop(mut maze: Vec<Vec<u8>>, starting_pos: (usize, usize)) -> bool {
    let max_steps = maze.len() * maze[0].len() * 2;
    let mut direction = (-1, 0);
    let mut pos = starting_pos;
    let mut step_no = 0;
    loop {
        if step_no > max_steps {
            break true;
        }
        let x = pos.0.checked_add_signed(direction.0);
        let y = pos.1.checked_add_signed(direction.1);
        let (x, y) = match (x, y) {
            (Some(x), Some(y)) => (x, y),
            _ => break false,
        };

        if x >= maze.len() {
            break false;
        }
        if y >= maze[0].len() {
            break false;
        }

        let next_block = &maze[x][y];
        match next_block {
            b'#' => {
                direction = get_next_direction(direction);
                continue;
            }
            _ => {
                maze[pos.0][pos.1] = b'x';

                pos = (x, y);
                step_no += 1;
            }
        }
    }
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
