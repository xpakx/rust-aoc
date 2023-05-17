use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_star();
    second_star();
}

fn first_star() {
    let file_path = Path::new("./input.txt");
    let file =  match File::open(&file_path){
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot load file: {}", error);
            return;
        }
    };
    let lines = io::BufReader::new(file).lines();
    let mut result = 0;
    for line in lines {
        if let Ok(line) = line {
            result += get_result_for_line(&line);
        }
    }
    println!("Result: {}", result);
}


fn get_result_for_line(line: &String) -> u32 {
    let middle = line.len() / 2;
    let first_part = &line[..middle];
    let second_part = &line[middle..];
    for byte in first_part.bytes() {
        for byte2 in second_part.bytes() {
            if byte == byte2 {
                let result = match byte {
                   b if (b>=60) && (b<=95) => (b as u32)-65+27,
                   b => (b as u32)-96
                };
                return result;
            }
        }
    }
    return 0
}

fn second_star() {
    let file_path = Path::new("./input.txt");
    let file =  match File::open(&file_path){
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot load file: {}", error);
            return;
        }
    };
    let mut lines = io::BufReader::new(file).lines();
    let mut result = 0;
    while let (Some(line), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        if let (Ok(line), Ok(line2), Ok(line3)) = (line, line2, line3) {
            result += get_badge(&line, &line2, &line3);
        }
    }
    println!("Result: {}", result);
}

fn get_badge(elve1: &String, elve2: &String, elve3: &String) -> u32 {
    for byte in elve1.bytes() {
        for byte2 in elve2.bytes() {
            for byte3 in elve3.bytes() {
                if byte == byte2 && byte == byte3 {
                    let result = match byte {
                        b if (b>=60) && (b<=95) => (b as u32)-65+27,
                        b => (b as u32)-96
                    };
                    return result;
                }
            }
        }
    }
    return 0;
}

