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
        if let Ok(ln) = line {
            let mut split = ln.split(" ");
            let elve_choice = split.next();
            let strategy = split.next();
            if let (Some(elve), Some(strategy)) = (elve_choice, strategy) {
                let shape_bonus = match strategy {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => 0
                };
                let result_bonus = match (elve, strategy) {
                    ("A", "X") => 3,
                    ("A", "Y") => 6,
                    ("B", "Y") => 3,
                    ("B", "Z") => 6,
                    ("C", "X") => 6,
                    ("C", "Z") => 3,
                    (_, _) => 0
                };
                result = result + result_bonus + shape_bonus;
            }
        }
    }
    println!("Result: {}", result);
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
    let lines = io::BufReader::new(file).lines();
    let mut result = 0;
    for line in lines {
        if let Ok(ln) = line {
            let mut split = ln.split(" ");
            let elve_choice = split.next();
            let strategy = split.next();
            if let (Some(elve), Some(strategy)) = (elve_choice, strategy) {
                let shape_bonus = match (elve, strategy) {
                    ("A", "X") => 3,
                    ("A", "Y") => 1,
                    ("A", "Z") => 2,
                    ("B", "X") => 1,
                    ("B", "Y") => 2,
                    ("B", "Z") => 3,
                    ("C", "X") => 2,
                    ("C", "Y") => 3,
                    ("C", "Z") => 1,
                    (_, _) => 0
                };
                let result_bonus = match strategy {
                    "Y" => 3,
                    "Z" => 6,
                    _ => 0
                };
                result = result + result_bonus + shape_bonus;
            }
        }
    }
    println!("Result: {}", result);
}
