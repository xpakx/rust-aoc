use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let input = read_input()
        .expect("Should read input from file");
    let (map, moves) = input
        .split_once("\n\n")
        .expect("Input should contain both map and moves");
    let instructions: Vec<Move> = parse_moves(moves);
    let map: Vec<Vec<Tile>> = parse_map(map);
    println!("{:?}", instructions);
    for row in map.iter() {
        for tile in row.iter() {
            print!(" {} ", match tile {
                Tile::Nothing => ' ',
                Tile::Wall => '#',
                Tile::Floor => '.'
            });
        }
        println!("");
    }
    let height = first_star();
    println!("Height: {}", height);
    let height = second_star();
    println!("Height: {}", height);
}

fn first_star() -> usize {
    0
}

fn second_star() -> usize {
    0
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

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut num = 0;
    for c in input.trim().chars() {
        if c.is_numeric() {
            let current = c.to_digit(10).unwrap();
            num *= 10;
            num += current;
        } else {
            moves.push(Move::Forward(num));
            num = 0;
            let rotate = match c {
                'R' => Move::Rotate(Rotation::Right),
                _ => Move::Rotate(Rotation::Left),
            };
            moves.push(rotate);
        }
    }
    if num > 0 {
        moves.push(Move::Forward(num));
    }
    moves
}
fn parse_map(input: &str) -> Vec<Vec<Tile>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Floor,
                _ => Tile::Nothing,
            };
            row.push(tile);
        }
        result.push(row);
    }
    result
}

enum Tile {
    Wall,
    Floor,
    Nothing,
}

#[derive(Debug)]
enum Move {
    Forward(u32),
    Rotate(Rotation),
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
} 
