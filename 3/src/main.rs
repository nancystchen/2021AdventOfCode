use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::Chars;

#[derive(Debug)]
struct Node {
    one: Option<Box<Node>>,
    zero: Option<Box<Node>>,
    value: String,
    weight: u32,
}

impl Node {
    fn new(value: String) -> Self {
        Self {
            one: None,
            zero: None,
            value,
            weight: 0,
        }
    }

    fn find(&self, find_most_common_bit: bool) -> String {
        if let Some(ref node_one) = self.one {
            if let Some(ref node_zero) = self.zero {
                if node_one.weight < node_zero.weight {
                    if find_most_common_bit {
                        node_zero.find(find_most_common_bit)
                    } else {
                        node_one.find(find_most_common_bit)
                    }
                } else {
                    if find_most_common_bit {
                        node_one.find(find_most_common_bit)
                    } else {
                        node_zero.find(find_most_common_bit)
                    }
                }
            } else {
                node_one.find(find_most_common_bit)
            }
        } else if let Some(ref node_zero) = self.zero {
            node_zero.find(find_most_common_bit)
        } else {
            // leaf node with full path
            self.value.clone()
        }
    }
}

fn add_node_to_bst(root: &mut Node, path: &mut Chars) {
    match path.next() {
        Some('1') => {
            if let Some(ref mut node) = root.one {
                let old_weight = node.weight;
                add_node_to_bst(node, path);
                root.weight += node.weight - old_weight;
            } else {
                let mut new_str = root.value.clone();
                new_str.push('1');
                let mut new_node = Node::new(new_str);
                add_node_to_bst(&mut new_node, path);
                root.weight += 1 + new_node.weight;
                root.one = Some(Box::new(new_node));
            }
        }
        Some('0') => {
            if let Some(ref mut node) = root.zero {
                let old_weight = node.weight;
                add_node_to_bst(node, path);
                root.weight += node.weight - old_weight;
            } else {
                let mut new_str = root.value.clone();
                new_str.push('0');
                let mut new_node = Node::new(new_str);
                add_node_to_bst(&mut new_node, path);
                root.weight += 1 + new_node.weight;
                root.zero = Some(Box::new(new_node));
            }
        }
        _ => {}
    };
}

fn from_binary_str_to_decimal(binary_str: &str) -> i32 {
    binary_str
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, ch)| {
            let bit: u32 = ch.into();
            acc + 2_i32.pow(idx as u32) * ((bit - 48) as i32)
        })
}

fn calculate_oxygen_generator_rating(bst: &Node) -> i32 {
    let o2_str = bst.find(true);
    println!("{}", o2_str);
    from_binary_str_to_decimal(&o2_str)
}

fn calculate_co2_srubber_rating(bst: &Node) -> i32 {
    let co2_str = bst.find(false);
    println!("{}", co2_str);
    from_binary_str_to_decimal(&co2_str)
}

fn calculate_epsilon_rate(gemma_rate: &str) -> String {
    String::from_iter(
        gemma_rate
            .chars()
            .map(|ch| if ch == '1' { '0' } else { '1' }),
    )
}

fn calculate_gemma_rate(data: Lines<BufReader<File>>, root: &mut Node) -> String {
    let mut lines = data.peekable();
    let mut num_one_in_binary_position: Vec<usize> = lines
        .peek()
        .expect("Cannot find the first line of data")
        .as_ref()
        .map(|line| vec![0; line.len()])
        .unwrap();
    let mut total_line_count = 0;
    lines.enumerate().for_each(|(idx, line)| {
        let binary_str: String = line.expect("Cannot parse data into binary string");
        let mut chars_iter = binary_str.chars();
        add_node_to_bst(root, &mut chars_iter);
        println!("{}", root.weight);
        binary_str.chars().enumerate().for_each(|(idx, c)| {
            if c == '1' {
                num_one_in_binary_position[idx] += 1;
            }
        });
        total_line_count = idx + 1;
    });

    num_one_in_binary_position
        .into_iter()
        .fold(String::new(), |mut acc, count| {
            if count < (total_line_count / 2) {
                acc.push('0');
            } else {
                acc.push('1');
            }
            acc
        })
}

fn get_data(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).expect("Cannot open file");
    BufReader::new(file).lines()
}

fn main() {
    println!("Solving problems...");
    let lines = get_data("input.txt");
    let mut bst = Node {
        one: None,
        zero: None,
        value: String::new(),
        weight: 0,
    };
    let gemma_rate_str = calculate_gemma_rate(lines, &mut bst);
    println!("{}", gemma_rate_str);
    let epsilon_rate_str = calculate_epsilon_rate(&gemma_rate_str);

    let gemma_rate = from_binary_str_to_decimal(&gemma_rate_str);
    let epsilon_rate = from_binary_str_to_decimal(&epsilon_rate_str);

    println!(
        "Gemma rate: {}. Epsilon rate: {}. Power of rates: {}",
        gemma_rate,
        epsilon_rate,
        gemma_rate * epsilon_rate
    );

    let o2_rate = calculate_oxygen_generator_rating(&bst);
    let co2_rate = calculate_co2_srubber_rating(&bst);

    println!(
        "o2: {}, co2: {}, life support rating: {}",
        o2_rate,
        co2_rate,
        o2_rate * co2_rate
    );
}
