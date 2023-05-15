use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    second_star();
}

fn first_star() {
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

fn second_star() {
    const FILE_PATH: &str = "./input.txt";
    let file = File::open(FILE_PATH).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut current_elve_calories: u32 = 0;
    let mut max_elves_calories: [u32; 3] = [0,0,0];
    for line in lines {
        let curr_line = line.unwrap();
        if curr_line == "" {
            if current_elve_calories > max_elves_calories[0] {
                max_elves_calories[2] = max_elves_calories[1];
                max_elves_calories[1] = max_elves_calories[0];
                max_elves_calories[0] = current_elve_calories;
            } else if current_elve_calories > max_elves_calories[1] {
                max_elves_calories[2] = max_elves_calories[1];
                max_elves_calories[1] = current_elve_calories;
            } else if current_elve_calories > max_elves_calories[2] {
                max_elves_calories[2] = current_elve_calories;
            }

            current_elve_calories = 0;
        }
        else {
            let calories: u32 = curr_line.parse().unwrap();
            current_elve_calories = current_elve_calories + calories;
        }
    }
    if current_elve_calories > max_elves_calories[0] {
        max_elves_calories[0] = current_elve_calories;
    }
    println!("#1 result: {}", max_elves_calories[0]);
    println!("#2 result: {}", max_elves_calories[1]);
    println!("#3 result: {}", max_elves_calories[2]);

    let mut sum: u32 = 0;
    for item in max_elves_calories {
        sum = sum + item;
    }
    println!("\nSum: {}", sum);
}

