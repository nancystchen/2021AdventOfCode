use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn polymerize_too_big(template: String, rules: &HashMap<String, char>) -> String {
    let chars = template.chars().collect::<Vec<char>>();
    let mut polymer = (0..template.len() - 1)
        .map(|first_idx| {
            let first = chars[first_idx];
            let second = chars[first_idx + 1];
            let key = [first, second].iter().collect::<String>();
            if let Some(val) = rules.get(&key) {
                [first, *val].iter().collect()
            } else {
                "".to_owned()
            }
        })
        .collect::<Vec<String>>();
    let last_char = String::from(chars[template.len() - 1]);
    polymer.push(last_char);
    polymer.join("")
}

fn polymerize(
    front: char,
    back: char,
    rules: &HashMap<String, char>,
    counts: &mut HashMap<char, usize>,
    step: usize,
) {
    let key = [front, back].iter().collect::<String>();
    if let Some(val) = rules.get(&key) {
        let count = counts.entry(*val).or_insert(0);
        // new polymer element added
        *count += 1;
        if step > 0 {
            polymerize(front, *val, rules, counts, step - 1);
            polymerize(*val, back, rules, counts, step - 1);
        }
    }
}

fn parse_data(
    mut data: Lines<BufReader<File>>,
    counts: &mut HashMap<char, usize>,
) -> (Vec<(char, char)>, HashMap<String, char>) {
    let mut rules = HashMap::<String, char>::new();
    let template = data.next().unwrap().unwrap();
    data.next();
    data.for_each(|line| {
        if let Ok(string) = line {
            let trimmed_str = string.replace(" -> ", ",");
            let mut split = trimmed_str.split(',');
            rules.insert(
                split.next().unwrap().to_owned(),
                split.next().unwrap().chars().next().unwrap(),
            );
        }
    });

    let temp_chars = template.chars().collect::<Vec<char>>();
    let pairs = (0..temp_chars.len() - 1)
        .map(|i| (temp_chars[i], temp_chars[i + 1]))
        .collect();
    temp_chars.iter().for_each(|ch| {
        let count = counts.entry(*ch).or_insert(0);
        *count += 1;
    });
    (pairs, rules)
}

fn calculate_diff(counts: &HashMap<char, usize>) -> usize {
    let (max, min) = counts
        .values()
        .fold((0, usize::MAX), |(mut max, mut min), &val| {
            if val < min {
                min = val;
            } else if val > max {
                max = val;
            }
            (max, min)
        });
    println!("{} {}", max, min);
    max - min
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let mut counts = HashMap::<char, usize>::new();
    let (pairs, rules) = parse_data(data, &mut counts);
    let step = 40;
    pairs
        .iter()
        .for_each(|(front, back)| polymerize(*front, *back, &rules, &mut counts, step - 1));
    println!(
        "Diff between most and least common element: {}",
        calculate_diff(&counts)
    );
}
