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
    for line in lines {
        if let Ok(line) = line {
            println!("{}", line);
            let mut splitted = line.split(" ");
            let command = splitted.next().unwrap();
            let command = match command {
                "$" => splitted.next().unwrap(),
                x => x
            };
            let target = match command {
                "cd" | "dir" => Some(splitted.next().unwrap()),
                _ => None
            };

        }
    }
}
