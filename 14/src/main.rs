use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn polymerize(template: String, rules: &HashMap<String, char>) -> String {
    let chars = template.chars().collect::<Vec<char>>();
    let polymer = (0..template.len() - 2)
        .map(|first_idx| {
            let first = chars[first_idx];
            let second = chars[first_idx + 1];
            let key = [first, second].iter().collect::<String>();
            if let Some(val) = rules.get(&key) {
                [first, *val, second].iter().collect()
            } else {
                "".to_owned()
            }
        })
        .collect::<Vec<String>>();
    polymer.join("")
}

fn parse_data(mut data: Lines<BufReader<File>>) -> (String, HashMap<String, char>) {
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

    (template, rules)
}

fn calculate_diff(polymer: &str) -> usize {
    unimplemented!()
}

fn main() {
    let file = File::open("sample_input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let (template, rules) = parse_data(data);
    let polymer = (0..10).fold(template, |t, _| polymerize(t, &rules));
    println!(
        "Diff between most and least common element: {}",
        calculate_diff(&polymer)
    );
}
