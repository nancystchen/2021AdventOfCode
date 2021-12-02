use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Lines};
use std::str::FromStr;

enum Data {
    Forward(u32),
    Downward(u32),
    Up(u32),
}

#[derive(Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse data: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

impl FromStr for Data {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split(' ').collect::<Vec<&str>>();
        let direction = data[0];
        let val = data[1].parse::<u32>().unwrap();
        if direction == "forward" {
            return Ok(Self::Forward(val));
        } else if direction == "down" {
            return Ok(Self::Downward(val));
        } else if direction == "up" {
            return Ok(Self::Up(val));
        }
        Err(ParseError(s.to_owned()))
    }
}

fn calculate_course(lines: Lines<BufReader<File>>) -> (u32, u32) {
    let mut x = 0;
    let mut y = 0;
    lines.for_each(|line| {
        if let Ok(data_string) = line {
            match Data::from_str(&data_string).expect("Error parsing data") {
                Data::Forward(x1) => x = x + x1,
                Data::Downward(y1) => y = y + y1,
                Data::Up(y2) => y = y - y2,
            }
        }
    });
    (x, y)
}

fn get_data(file_path: &str) -> Result<Lines<BufReader<File>>, Error> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file).lines())
}

fn main() -> std::io::Result<()> {
    let lines = get_data("input.txt")?;
    let (final_horizontal_pos, final_depth) = calculate_course(lines);
    println!(
        "Horizontal: {}, Depth: {}",
        final_horizontal_pos, final_depth
    );
    println!("Multiplied value: {}", final_horizontal_pos * final_depth);
    Ok(())
}
