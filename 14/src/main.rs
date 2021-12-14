use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn polymerize(template: String, rules: &HashMap<String, String>) -> String {
    unimplemented!()
}

fn parse_data(data: Lines<BufReader<File>>) -> (String, HashMap<String, String>) {
    unimplemented!()
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
