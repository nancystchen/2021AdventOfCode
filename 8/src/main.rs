use std::fs::File;
use std::io::{BufRead, BufReader};

// Given a sequence and encoding, return the number decoded from sequence.
// `encoding` represents wire placement seven-segment number display as the following:
//
//  0000
// 1    2
// 1    2
//  3333
// 4    5
// 4    5
//  6666
//
fn decode_sequence(sequence: &str, encoding: &[String]) -> usize {
    let mut chars = sequence.chars().into_iter().collect::<Vec<char>>();
    chars.sort_unstable();
    let sorted_sequence = chars.into_iter().collect::<String>();
    encoding
        .iter()
        .position(|s| s == &sorted_sequence)
        .expect(&format!("Failed to decode {}!", sequence))
}

// Given a list of strings, find characters that are not present in every string.
fn find_different_segments(patterns: &[&str]) -> String {
    let common_segments = find_common_segments(patterns);
    patterns
        .iter()
        .fold(vec![], |mut acc, &p| {
            p.chars().for_each(|ch| {
                if !common_segments.contains(ch) && !acc.contains(&ch) {
                    acc.push(ch)
                }
            });
            acc
        })
        .iter()
        .collect()
}

// Given a list of strings, find intersecting characters.
fn find_common_segments(patterns: &[&str]) -> String {
    let p0 = patterns[0].to_owned();
    patterns.iter().fold(p0, |acc, &p| {
        p.chars().filter(|&ch| acc.contains(ch)).collect()
    })
}

// Given seven-segment patterens for 10 digits, find the encoding of segments and return each
// digit's encoding.
fn deduce_encoding(patterns: &str) -> Vec<String> {
    // need two tables:
    // encoding table shows what number corresponds to what
    // segment table shows which signal belongs to which segment
    // then:
    // s0 = diff(1,7)
    // s5 = diff(1,8), 0 || 6 || 9) == 1
    // s2 = diff(1,s5)
    // s1 = diff(diff(same(2 || 3 || 5 || 9), 1), 4)
    // s4 = same(diff(diff(same(2 || 3 || 5 || 9), 1), 4)
    // s3 = diff(s0 + s1 + s2 + s5,  2 || 3 || 5 || 9) == 1
    // s6 = last segment!
    unimplemented!()
}

fn part_two() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let total_decoded_number_sum = lines
        .map(|line| {
            let string = line.unwrap();
            let chunks = string.split('|').collect::<Vec<&str>>();
            let pattern = chunks.first().unwrap();
            let sequence = chunks.last().unwrap();
            let encoding = deduce_encoding(pattern.trim());
            decode_sequence(sequence.trim_start(), &encoding)
        })
        .sum::<usize>();

    println!("{}", total_decoded_number_sum);
}

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

    //part_two();
    println!("{}", find_different_segments(&["acd", "dc", "cb"]));
}
