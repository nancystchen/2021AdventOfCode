use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn count_increased_depth(lines: Lines<BufReader<File>>) -> u32 {
    let mut prev_depth = std::u32::MAX;
    let mut count = 0;
    for line in lines {
        if let Ok(depth_string) = line {
            let depth = depth_string.parse::<u32>().unwrap();
            if prev_depth < depth {
                count = count + 1;
            }
            prev_depth = depth;
        }
    }
    count
}

fn count_increased_sliding_window(lines: Lines<BufReader<File>>) -> u32 {
    // first add up first 3
    // then remove first, add last, compare
    // keep doing it and put on counter
    // when finish last available number, stop
    let mut count = 0;
    let mut sum = 0;
    let mut first = 0;
    let mut second = 0;
    lines.enumerate().for_each(|(idx, line)| {
        if let Ok(depth_string) = line {
            let val = depth_string.parse::<u32>().unwrap();
            if idx == 0 {
                first = val;
            } else if idx == 1 {
                second = val;
            } else if idx == 2 {
                sum = first + second + val;
                first = second;
                second = val;
            } else {
                let new_sum = first + second + val;
                if new_sum > sum {
                    count = count + 1;
                }
                sum = new_sum;
                first = second;
                second = val;
            }
        }
    });
    count
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    //let count = count_increased_depth(lines());
    //println!("Depth increased: {} times", count);
    let count = count_increased_sliding_window(lines);
    println!("Depth sliding window increased: {} times", count);
    Ok(())
}
