use std::fs;
use std::path::Path;
use std::io;
use std::collections::HashMap;

fn main() {
    first_star();
    second_star();
}

fn first_star() {
    let input = read_input().expect("Should read input from file");
    let directions: Vec<Direction> = parse_input(&input);
}

fn second_star() {
}

fn read_input() -> Result<String, io::Error> {
    let file_path = Path::new("./input.txt");
    let file =  match fs::read_to_string(&file_path){
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(file);
}

enum Direction {
    Left,
    Right
}

fn parse_input(input: &String) -> Vec<Direction> {
    input
        .chars()
        .filter(|c| c == &'>' || c == &'<')
        .map(|c| match c {
            '>' => Direction::Right,
            _ => Direction::Left
        })
        .collect()
}

enum Polyomino {
    ITetromino,
    I90Tetromino,
    OTetromino,
    JPentomino,
    XPentomino
}
