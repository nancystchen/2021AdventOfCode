use std::io::BufRead;

const JUVENILE_REPRO_CYCLE: u32 = 9;
const ADULT_REPRO_CYCLE: u32 = 7;

#[derive(Debug)]
struct LanternFish {
    days_left_in_cycle: u32,
    juvenile: bool,
}

impl LanternFish {
    fn new(days_left_in_cycle: u32, juvenile: bool) -> Self {
        Self {
            days_left_in_cycle,
            juvenile,
        }
    }

    // A lanternfish matures from juvenile to adult
    fn matures(&mut self) {
        self.juvenile = false;
        self.days_left_in_cycle = ADULT_REPRO_CYCLE;
    }

    // Given a lanternfish, count how many offsprings it will reproduce
    // in a given time period. The offsprings of its offspring counts, too.
    fn reproduce(&mut self, time_period: u32) -> u64 {
        if time_period > self.days_left_in_cycle {
            let time_to_reproduce = time_period - self.days_left_in_cycle;
            if self.juvenile {
                self.matures();
            } else {
                self.days_left_in_cycle = ADULT_REPRO_CYCLE;
            }
            1 + self.reproduce(time_to_reproduce)
                + LanternFish::new(JUVENILE_REPRO_CYCLE, true).reproduce(time_to_reproduce)
        } else {
            0
        }
    }
}

fn part_two(numbers: &mut Vec<usize>, days: usize) -> usize {
    let mut new_fish_starting_idx = numbers.len();
    (0..days).for_each(|d| {
        let mut new_fish = 0;
        println!("day {}",  d + 1);
        numbers.iter_mut().enumerate().for_each(|(idx, n)| {
            if *n > 0 {
                *n -= 1;
            } else if idx <= new_fish_starting_idx {
                *n = 6 as usize;
                new_fish += 1;
            }
        });
        numbers.append(&mut (0..new_fish).map(|_| 8).collect::<Vec<usize>>());
        new_fish_starting_idx = numbers.len();
    });
    new_fish_starting_idx
}

fn make_adult_lanternfish(cycles: Vec<usize>) -> Vec<LanternFish> {
    cycles
        .iter()
        .map(|cycle| LanternFish::new(*cycle as u32, false))
        .collect()
}

fn get_data(line: String) -> Vec<usize> {
    line.trim()
        .split(',')
        .map(|ch| ch.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let mut cycles = get_data(line);
    // part 1
    let time_period = 256;
    let lanternfish = make_adult_lanternfish(cycles.clone());
    let total_fish_count: u64 = lanternfish.len() as u64
        + lanternfish
            .into_iter()
            .enumerate()
            .map(|(idx, mut fish)| {
                println!("checking fish no.{}", idx);
                fish.reproduce(time_period)})
            .sum::<u64>();
    println!(
        "Total number of lanternfish after {} days: {}",
        time_period, total_fish_count
    );

    //let total = part_two(&mut cycles, 80);
    //println!("Total number of lanternfish after 256 days: {}", total);
}
