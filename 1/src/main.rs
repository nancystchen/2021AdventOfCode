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

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let count = count_increased_depth(lines);
    println!("Depth increased: {} times", count);
    Ok(())
}
