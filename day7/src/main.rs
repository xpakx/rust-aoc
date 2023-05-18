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
    let mut stack = Vec::new();
    let mut result = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut splitted = line.split(" ");
            let command = splitted.next().unwrap();
            let command = match command {
                "$" => splitted.next().unwrap(),
                x => x
            };
            if command == "ls" || command == "dir" {
                continue;
            }
            let target = match command {
                "cd" | "dir" => Some(splitted.next().unwrap()),
                _ => Some(splitted.next().unwrap())
            };
            if command == "cd" {
                let new_dir = target.unwrap();
                if new_dir == ".." {
                    if let Some(value) = stack.pop() {
                        if value <= 100000 {
                            result += value;
                        }
                        let last = stack.len()-1;
                        stack[last] += value;
                    }
                } else {
                    stack.push(0);
                }
            } else {
                let file_size: usize = command.parse().unwrap();
                let last = stack.len()-1;
                stack[last] += file_size;
            }
        }
    }
    println!("{}", result);
}

