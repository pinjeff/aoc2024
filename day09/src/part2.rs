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

    let mut disk: Vec<(Option<usize>, usize)> = vec![];
    let mut file_id = 0;
    let mut file_mode = true;
    for n in input {
        match file_mode {
            true => {
                disk.push((Some(file_id), n));
                file_id += 1;
            }
            false => {
                disk.push((None, n));
            }
        }
        file_mode = !file_mode;
    }

    file_id -= 1;
    while file_id != 0 {
        let block_idx = disk.iter().position(|s| s.0 == Some(file_id)).unwrap();
        let block_len = disk[block_idx].1;
        let target_slot_idx = disk[0..block_idx]
            .iter()
            .position(|s| s.0 == None && s.1 >= block_len);

        if let Some(slot_idx) = target_slot_idx {
            let slot = &mut disk[slot_idx];
            slot.1 -= block_len;

            let block = disk.remove(block_idx);
            disk.insert(block_idx, (None, block_len));
            disk.insert(slot_idx, block);
        };

        file_id -= 1;
    }

    let checksum: usize = disk
        .iter()
        .map(|s| match s.0 {
            Some(fd) => (0..s.1).into_iter().map(|_| fd).collect::<Vec<_>>(),
            None => (0..s.1).into_iter().map(|_| 0).collect(),
        })
        .flatten()
        .enumerate()
        .map(|(i, n)| i * n)
        .sum();

    println!("Checksum: {:?}", checksum);
    Ok(())
}
