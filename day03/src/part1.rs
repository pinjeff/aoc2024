use regex::Regex;
use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;
    for (_s, [lhs, rhs]) in re.captures_iter(&content).map(|c| c.extract()) {
        result += lhs.parse::<usize>()? * rhs.parse::<usize>()?;
    }

    println!("Result: {result:?}");
    Ok(())
}
