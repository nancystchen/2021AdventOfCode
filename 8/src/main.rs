use std::fs::File;
use std::io::{BufRead, BufReader};

fn sort_string(string: &str) -> String {
    let mut chars = string.chars().into_iter().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.iter().collect()
}

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
    let num = sequence.split(' ').fold(0, |acc, s| {
        let mut chars = s.chars().into_iter().collect::<Vec<char>>();
        chars.sort_unstable();
        let seq = chars.into_iter().collect::<String>();
        let idx = encoding
            .iter()
            .position(|s| s == &seq)
            .expect(&format!("Failed to decode sequence `{}`!", s));
        acc * 10 + idx
    });
    println!("{}", num);
    num
}

// Given a list of strings, find characters that are not present in every string.
fn find_different_segments(patterns: &[&str]) -> Vec<char> {
    let common_segments = find_common_segments(patterns);
    patterns.iter().fold(vec![], |mut acc, &p| {
        p.chars().for_each(|ch| {
            if !common_segments.contains(ch) && !acc.contains(&ch) {
                acc.push(ch)
            }
        });
        acc
    })
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
    let mut encoding_table = vec!["".to_owned(); 10];
    // segment table shows which signal belongs to which segment
    let mut segment_table = vec![' '; 7];
    let mut five_segment_patterns = vec![];
    let mut six_segment_patterns = vec![];
    patterns.split(' ').for_each(|p| {
        if p.len() == 2 {
            // found `1`
            encoding_table[1] = sort_string(p);
        } else if p.len() == 3 {
            // found `7`
            encoding_table[7] = sort_string(p);
        } else if p.len() == 4 {
            // found `4`
            encoding_table[4] = sort_string(p);
        } else if p.len() == 5 {
            five_segment_patterns.push(p);
        } else if p.len() == 6 {
            six_segment_patterns.push(p);
        } else if p.len() == 7 {
            // found `8`
            encoding_table[8] = sort_string(p);
        }
    });
    // then:
    // s0 = diff(1,7)
    segment_table[0] = *find_different_segments(&[&encoding_table[1], &encoding_table[7]])
        .first()
        .unwrap();
    // s5 = diff(1,8), 0 || 6 || 9) == 1
    let diff_1_8 = find_different_segments(&[&encoding_table[1], &encoding_table[8]])
        .iter()
        .collect::<String>();
    segment_table[5] = *six_segment_patterns
        .iter()
        .filter_map(|p| {
            let diff = find_different_segments(&[p, &diff_1_8]);
            if diff.len() == 1 {
                Some(*diff.first().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<char>>()
        .first()
        .unwrap();

    // s2 = diff(1,s5)
    segment_table[2] =
        *find_different_segments(&[&encoding_table[1], &String::from(segment_table[5])])
            .first()
            .unwrap();
    // s3 = diff(same(2 || 3 || 5), diff_1_4)
    let diff_1_4 = find_different_segments(&[&encoding_table[1], &encoding_table[4]])
        .iter()
        .collect::<String>();
    segment_table[3] =
        find_common_segments(&[&diff_1_4, &find_common_segments(&five_segment_patterns)])
            .chars()
            .next()
            .unwrap();
    // s1 = diff(diff_1,4, s3)
    segment_table[1] = *find_different_segments(&[&diff_1_4, &String::from(segment_table[3])])
        .first()
        .unwrap();
    // s4 = diff(diff(2 || 3 || 5), s1_s2_s5)
    segment_table[4] = *find_different_segments(&[
        &[segment_table[1], segment_table[2], segment_table[5]]
            .iter()
            .collect::<String>(),
        &find_different_segments(&five_segment_patterns)
            .iter()
            .collect::<String>(),
    ])
    .first()
    .unwrap();
    // s6 = last segment!
    segment_table[6] = *find_different_segments(&[
        &(0..6).map(|i| segment_table[i]).collect::<String>(),
        "abcdefg",
    ])
    .first()
    .unwrap();
    println!("segment table: {:?}", segment_table);

    // finally build encoding_table
    // for 0
    let mut zero = [
        segment_table[0],
        segment_table[1],
        segment_table[2],
        segment_table[4],
        segment_table[5],
        segment_table[6],
    ];
    zero.sort_unstable();
    encoding_table[0] = zero.iter().collect();
    // for 2
    let mut two = [
        segment_table[0],
        segment_table[2],
        segment_table[3],
        segment_table[4],
        segment_table[6],
    ];
    two.sort_unstable();
    encoding_table[2] = two.iter().collect();
    // for 3
    let mut three = [
        segment_table[0],
        segment_table[2],
        segment_table[3],
        segment_table[5],
        segment_table[6],
    ];
    three.sort_unstable();
    encoding_table[3] = three.iter().collect();
    // for 5
    let mut five = [
        segment_table[0],
        segment_table[1],
        segment_table[3],
        segment_table[5],
        segment_table[6],
    ];
    five.sort_unstable();
    encoding_table[5] = five.iter().collect();
    // for 6
    let mut six = [
        segment_table[0],
        segment_table[1],
        segment_table[3],
        segment_table[4],
        segment_table[5],
        segment_table[6],
    ];
    six.sort_unstable();
    encoding_table[6] = six.iter().collect();
    // for 9
    let mut nine = [
        segment_table[0],
        segment_table[1],
        segment_table[2],
        segment_table[3],
        segment_table[5],
        segment_table[6],
    ];
    nine.sort_unstable();
    encoding_table[9] = nine.iter().collect();

    println!("encoding table: {:?}", encoding_table);
    encoding_table
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

    part_two();
}
