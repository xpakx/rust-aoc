use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    const FILE_PATH: &str = "./input.txt";
    let file = File::open(FILE_PATH).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut current_elve: u32 = 0;
    let mut current_elve_calories: u32 = 0;
    let mut max_elve: u32 = 0;
    let mut max_elve_calories: u32 = 0;
    for line in lines {
        let curr_line = line.unwrap();
        if curr_line == "" {
            if current_elve_calories > max_elve_calories {
                max_elve = current_elve;
                max_elve_calories = current_elve_calories;
            }
            current_elve = current_elve + 1;
            current_elve_calories = 0;
        }
        else {
            let calories: u32 = curr_line.parse().unwrap();
            current_elve_calories = current_elve_calories + calories;
        }
    }
    if current_elve_calories > max_elve_calories {
        max_elve = current_elve;
        max_elve_calories = current_elve_calories;
    }
    println!("elf #{} has most calories, {}", max_elve, max_elve_calories);
}
