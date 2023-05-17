use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = Path::new("./input.txt");
    let file =  match File::open(&file_path){
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot load file: {}", error);
            return;
        }
    };
    let lines = io::BufReader::new(file).lines();
    let mut first_star = 0;
    let mut second_star = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut split = line.split(",");
            let first = split.next();
            let second = split.next();
            if let (Some(first), Some(second)) = (first, second) {
                let first = to_range(first);
                let second = to_range(second);
                if contains(first, second) || contains(second, first) {
                    first_star += 1;
                }
                if overlap(first, second) || overlap(second, first) {
                    second_star += 1
                }
            }
        }
    }
    println!("First star: {}", first_star);
    println!("Second star: {}", second_star);
}

fn to_range(text: &str) -> (u32, u32) {
    let mut split = text.split("-");
    if let (Some(first), Some(second)) = (split.next(), split.next()) {
        if let (Ok(first), Ok(second)) = (first.parse::<u32>(), second.parse::<u32>()) {
            return (first, second);
        }
    }
    return (0, 0);
}

fn contains(first: (u32, u32), second: (u32, u32)) -> bool {
    return first.0<=second.0 && first.1>=second.1;
}

fn overlap(first: (u32, u32), second: (u32, u32)) -> bool {
    return (second.1 >= first.0 && second.1 <= first.1) ||
        (second.0 >= first.0 && second.0 <= first.1) 
}

