use regex::Regex;
use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut split: Vec<_> = content.split("don't()").collect();
    let mut truncated_string = String::from(split[0]);
    split.remove(0);
    for text in split {
        if let Some(do_match) = text.split_once("do()") {
            truncated_string.push_str(do_match.1);
        }
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;
    for (_s, [lhs, rhs]) in re.captures_iter(&truncated_string).map(|c| c.extract()) {
        result += lhs.parse::<usize>()? * rhs.parse::<usize>()?;
    }

    println!("Result: {result:?}");
    Ok(())
}
