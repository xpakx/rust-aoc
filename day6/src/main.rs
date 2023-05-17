use std::fs;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let file_path = Path::new("./input.txt");
    let input =  match fs::read_to_string(&file_path){
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot load file: {}", error);
            return;
        }
    };
    let bytes = input.as_bytes();

    for (i, c) in bytes.windows(4).enumerate() {
        let mut set = HashSet::new();
        for w in c {
            set.insert(w);
        }
        if set.len() == 4 {
            println!("Result: {}", i+4);
            return;
        }
    }
}
