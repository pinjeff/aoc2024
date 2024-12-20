use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let input: Vec<usize> = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| String::from(c).parse().expect(&format!("{c}")))
        })
        .flatten()
        .collect();

    let mut disk: Vec<Option<usize>> = vec![];
    let mut file_id = 0;
    let mut file_mode = true;
    for n in input {
        match file_mode {
            true => {
                disk.extend((0..n).into_iter().map(|_| Some(file_id)));
                file_id += 1;
            }
            false => {
                disk.extend((0..n).into_iter().map(|_| None));
            }
        }
        file_mode = !file_mode;
    }

    let file_blocks_count = disk.iter().filter(|b| b.is_some()).count();
    while !disk[0..file_blocks_count].iter().all(|b| b.is_some()) {
        let last_file_block_idx =
            disk.len() - 1 - disk.iter().rev().position(|b| b.is_some()).unwrap();
        let first_free_block_idx = disk.iter().position(|b| b.is_none()).unwrap();
        if last_file_block_idx < first_free_block_idx {
            break;
        }
        disk[first_free_block_idx] = disk[last_file_block_idx];
        disk[last_file_block_idx] = None;
    }

    let checksum: usize = disk
        .iter()
        .enumerate()
        .map(|(i, b)| i * b.unwrap_or(0))
        .sum();

    println!("Checksum: {}", checksum);
    Ok(())
}
