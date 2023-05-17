use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    exercise(true);
    exercise(false);
}

fn exercise(first_star: bool) {
    let file_path = Path::new("./input.txt");
    let file =  match File::open(&file_path){
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot load file: {}", error);
            return;
        }
    };
    let mut lines = io::BufReader::new(file).lines();
    let mut cargo: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            if line == "" {
                break;
            } else {
                let row = parse_line(&line);
                for (i, &c) in row.iter().enumerate() {
                    if c != ' ' {
                        let column = cargo.get_mut(i);
                        if let Some(column) = column {
                            if !c.is_numeric() {
                                column.insert(0, c);
                            }
                        } else {
                            for _ in 0..(i-cargo.len()+1) {
                                cargo.push(Vec::new());
                            }
                            if !c.is_numeric() {
                                cargo.get_mut(i).unwrap().insert(0, c);
                            }
                        }
                    }
                }
            }
        }
    }   
    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            let instruction = parse_instruction(&line);
            if first_star {
                for _ in 0..(instruction.0) {
                    let c = cargo.get_mut(instruction.1).unwrap().pop(); 
                    if let Some(c) = c {
                        cargo.get_mut(instruction.2).unwrap().push(c);
                    }
                }
            } else {
                let row = cargo
                    .get_mut(instruction.1)
                    .unwrap();

                let mut v: Vec<char> = row
                    .drain((row.len() -instruction.0 as usize)..)
                    .collect();
                cargo.get_mut(instruction.2).unwrap().append(&mut v);
            }
        }
    }
    for mut col in cargo {
        print!("{}", match col.pop() {Some(c) => c, None => ' '});
    }
    print!("\n");
}

fn parse_instruction(text: &String) -> (u32, usize, usize) {
    let mut elements = text.split(" ");
    let amount = elements.nth(1);
    let column_from = elements.nth(1);
    let column_to = elements.nth(1);
    if let (Some(a), Some(f), Some(t)) = (amount, column_from, column_to) {
        if let(Ok(a), Ok(f), Ok(t)) = (a.parse::<u32>(), f.parse::<usize>(), t.parse::<usize>()) {
            return (a,f-1,t-1);
        }
    }
    return (0, 0, 0);
}

fn parse_line(text: &String) -> Vec<char> {
    let mut parsed: Vec<char> = Vec::new();
    for (i, c) in text.chars().enumerate() {
        if i%4 == 1 {
            parsed.push(c);
        }
    }
    return parsed;
}

