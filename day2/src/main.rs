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
        if let Ok(ln) = line {
            println!("{}", ln);
            let mut split = ln.split(" ");
            let elve_choice = split.next();
            let strategy = split.next();
            if let (Some(elve), Some(strategy)) = (elve_choice, strategy) {
                println!("Elve: {}", elve);
                println!("Strategy: {}", strategy);
            }
        }
    }
    
}
