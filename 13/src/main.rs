use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

#[derive(Debug)]
struct Origami {
    dots: HashSet<(usize, usize)>,
}

impl Origami {
    fn new() -> Self {
        Self {
            dots: HashSet::<(usize, usize)>::new(),
        }
    }

    fn add_dot(&mut self, i: usize, j: usize) {
        self.dots.insert((i, j));
    }

    fn fold(&mut self, fold: &Fold) {
        let folded_dots = self
            .dots
            .iter()
            .map(|(i, j)| {
                let new_dot = Self::calculate_folded_dot(*i, *j, fold);
                ((*i, *j), new_dot)
            })
            .collect::<Vec<((usize, usize), (usize, usize))>>();
        folded_dots.into_iter().for_each(|(old_dot, new_dot)| {
            if old_dot != new_dot {
                self.dots.remove(&old_dot);
                self.dots.insert(new_dot);
            }
        });
    }

    fn calculate_folded_dot(i: usize, j: usize, fold: &Fold) -> (usize, usize) {
        match fold {
            Fold::Up(y) => {
                if j < *y {
                    (i, j)
                } else {
                    (i, y - (j - y))
                }
            }
            Fold::Left(x) => {
                if i < *x {
                    (i, j)
                } else {
                    (x - (i - x), j)
                }
            }
        }
    }
}

fn parse_data(data: Lines<BufReader<File>>) -> (Origami, Vec<Fold>) {
    let mut origami = Origami::new();
    let mut folds = vec![];
    let mut add_fold = false;
    for line in data.flatten() {
        if line.is_empty() {
            add_fold = true;
        } else if !add_fold {
            let dot = line
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            origami.add_dot(dot[0], dot[1]);
        } else {
            let string = line.replace("fold along ", "");
            let fold = string.split('=').collect::<Vec<&str>>();
            match fold[0] {
                "x" => folds.push(Fold::Left(fold[1].parse::<usize>().unwrap())),
                "y" => folds.push(Fold::Up(fold[1].parse::<usize>().unwrap())),
                _ => {}
            };
        }
    }
    (origami, folds)
}
fn make_graph(origami: &Origami) {
    let (max_i, max_j) =
        origami
            .dots
            .iter()
            .fold((0_usize, 0_usize), |(mut max_i, mut max_j), (i, j)| {
                if *i > max_i {
                    max_i = *i;
                }
                if *j > max_j {
                    max_j = *j;
                }
                (max_i, max_j)
            });
    println!("{} {}", max_i, max_j);

    (0..max_j + 1).for_each(|j| {
        (0..max_i + 1).for_each(|i| {
            if origami.dots.get(&(i, j)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!("");
    });
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let data = BufReader::new(file).lines();
    let (mut origami, folds) = parse_data(data);
    make_graph(&origami);
    folds.iter().for_each(|fold| {
        origami.fold(fold);
        println!("Visible dots: {}", origami.dots.len());
        make_graph(&origami);
    })
}
