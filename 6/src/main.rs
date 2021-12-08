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

fn make_adult_lanternfish(cycles: Vec<u32>) -> Vec<LanternFish> {
    cycles
        .iter()
        .map(|cycle| LanternFish::new(*cycle, false))
        .collect()
}

fn get_data(line: String) -> Vec<u32> {
    line.trim()
        .split(',')
        .map(|ch| ch.parse::<u32>().unwrap())
        .collect()
}

fn main() {
    println!("Solving problem...");
    let file = std::fs::File::open("sample_input.txt").unwrap();
    let line = std::io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let cycles = get_data(line);
    let time_period = 256;
    let lanternfish = make_adult_lanternfish(cycles);
    let total_fish_count: u64 = lanternfish.len() as u64 + lanternfish
        .into_iter()
        .map(|mut fish| fish.reproduce(time_period))
        .sum::<u64>();
    println!(
        "Total number of laternfish after {} days: {}",
        time_period, total_fish_count
    );
}
