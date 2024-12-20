use std::{cmp::Ordering, collections::HashMap, error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let (rules_lines, content_lines) = content.split_once("\n\n").unwrap();
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for rule_str in rules_lines.split('\n') {
        let (lhs, rhs) = rule_str.split_once('|').unwrap();
        let lhs: usize = lhs.parse().unwrap();
        let rhs: usize = rhs.parse().unwrap();
        rules_map
            .entry(lhs)
            .and_modify(|v| v.push(rhs))
            .or_insert(vec![rhs]);
    }

    let updates: Vec<Vec<usize>> = content_lines
        .split('\n')
        .into_iter()
        .filter(|l| *l != "")
        .map(|l| {
            l.split(',')
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let not_passing_updates: Vec<_> = updates
        .into_iter()
        .filter(|u| {
            !u.iter().enumerate().skip(1).all(|(i, n)| {
                u[0..i]
                    .iter()
                    .all(|p| rules_map.get(p).unwrap_or(&vec![]).contains(&n))
            })
        })
        .collect();

    let sorted_not_passing_updates: Vec<_> = not_passing_updates
        .into_iter()
        .map(|mut u| {
            u.sort_by(|a, b| {
                let a_bigger = rules_map.get(a).unwrap_or(&vec![]).contains(b);
                let b_bigger = rules_map.get(b).unwrap_or(&vec![]).contains(a);
                match (a_bigger, b_bigger) {
                    (true, _) => Ordering::Less,
                    (_, true) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            });
            u
        })
        .collect();

    let middle_page_sum: usize = sorted_not_passing_updates
        .iter()
        .map(|u| u[u.len() / 2])
        .sum();

    println!("Middle page sum: {middle_page_sum}");
    Ok(())
}
