use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn polymerize(template: String, rules: &HashMap<String, String>) -> String {
    unimplemented!()
}

fn parse_data(mut data: Lines<BufReader<File>>) -> (String, HashMap<String, String>) {
    let mut rules = HashMap::<String, String>::new();
    let template = data.next().unwrap().unwrap();
    data.next();
    data.for_each(|line| {
        if let Ok(string) = line {
            let trimmed_str = string.replace(" -> ", ",");
            let mut split = trimmed_str.split(',');
            rules.insert(
                split.next().unwrap().to_owned(),
                split.next().unwrap().to_owned(),
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
