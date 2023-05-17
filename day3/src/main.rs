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
