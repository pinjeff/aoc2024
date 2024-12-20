use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut reports: Vec<Vec<usize>> = vec![];

    for line in content.lines() {
        reports.push(
            line.split(" ")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|str| str.parse().unwrap())
                .collect(),
        )
    }

    let mut safe_reports_count = 0;
    for report in reports.iter() {
        let mut safe = false;
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            let xs: Vec<isize> = report
                .windows(2)
                .map(|slice| (slice[1] as isize - slice[0] as isize))
                .collect();

            let gradual = xs
                .iter()
                .map(|x| x.abs())
                .all(|diff| diff >= 1 && diff <= 3);

            let same_direction =
                xs.iter().sum::<isize>().abs() == xs.iter().map(|x| x.abs()).sum::<isize>();

            safe = safe || gradual && same_direction;
        }
        if safe {
            safe_reports_count += 1;
        }
    }

    println!("Safe Reports Count: {safe_reports_count}");
    Ok(())
}
