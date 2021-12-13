use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Heard this is gonna be a hard problem...");
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let simple_digit_counts = lines
        .map(|line| {
            //count number of 2, 3, or 7)
            line.unwrap()
                .split('|')
                .next_back()
                .unwrap()
                .trim_start()
                .split(' ')
                .filter(|sig| sig.len() == 2 || sig.len() == 3 || sig.len() == 4 || sig.len() == 7)
                .count()
        })
        .sum::<usize>();

    println!("{}", simple_digit_counts);
}
