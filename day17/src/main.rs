use std::fs;
use std::path::Path;
use std::io;
use std::collections::HashMap;
use std::cmp::min;

fn main() {
    first_star();
    second_star();
}

fn first_star() {
    let input = read_input().expect("Should read input from file");
    let directions: Vec<Direction> = parse_input(&input);
    let dir_len = directions.len();
    let mut board = vec![0b1111111];
    let mut dir = 0;
    for i in 0..2023 {
        let rock = get_polyomino(i);
        let mut mask = polyomino_to_bit_mask(&rock);
        let rock_height = mask.len();
        let mut stopped = false;
        let mut depth = 0;
        let mut free_fall = 3;
        println!("NEW ROCK FALLS");

        while !stopped {
            println!("{}", depth);
            let direction = directions[dir%dir_len];
            if no_wall(&mask, &direction, &rock) {
                let new_mask = if let Direction::Right = direction {
                    let mut new_mask = Vec::new();
                    for b in mask.iter() {
                        new_mask.push(b >> 1);
                    }
                    new_mask
                } else {
                    let mut new_mask = Vec::new();
                    for b in mask.iter() {
                        new_mask.push(b << 1);
                    }
                    new_mask
                };

                if depth == 0 || test_move(&board, depth, rock_height, &new_mask) {
                    mask = new_mask;
                } 
            }
            dir+= 1;
                println!("After dir");
    print_bits(&mask);

            if free_fall == 0 {
                // falling
                depth += 1;
                if test_move(&board, depth, rock_height, &mask) {
                    let height = board.len();
                    for i in 0..min(depth, rock_height) {
                        board[height-depth+i] = board[0] | mask[i];
                    }
                    for i in depth..rock_height {
                        board.push(mask[i]);
                    }


                    // append additional lines
                } else {
                    stopped = true;
                }
            } else {
                free_fall -= 1;
            }

        }
    print_bits(&board);
    }
    println!("Height: {}", board.len() - 1);
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

#[derive(Debug, Clone, Copy)]
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

fn get_polyomino(turn: usize) -> Polyomino {
    match turn%5 {
        0 => Polyomino::I90Tetromino,
        1 => Polyomino::XPentomino,
        2 => Polyomino::JPentomino,
        3 => Polyomino::ITetromino,
        _ => Polyomino::OTetromino
    }
}

fn polyomino_to_bit_mask(polyomino: &Polyomino) -> Vec<u8> {
    match polyomino {
        Polyomino::I90Tetromino => vec![
            0b0011110
        ],
        Polyomino::XPentomino => vec![
            0b0001000,
            0b0011100,
            0b0001000
        ],
        Polyomino::JPentomino => vec![
            0b0000100,
            0b0000100,
            0b0011100
        ],
        Polyomino::ITetromino => vec![
            0b0010000,
            0b0010000,
            0b0010000,
            0b0010000
        ],
        Polyomino::OTetromino => vec![
            0b0011000,
            0b0011000
        ]

    }
}

fn no_wall(mask: &Vec<u8>, dir: &Direction, shape: &Polyomino) -> bool {
    match dir {
        Direction::Right => {
            match shape {
                Polyomino::I90Tetromino => mask[0] != 0b0001111, 
                Polyomino::XPentomino => mask[1] != 0b0000111,
                Polyomino::JPentomino => mask[2] != 0b0000111,
                Polyomino::ITetromino => mask[0] != 0b0000001,
                Polyomino::OTetromino => mask[0] != 0b0000011
            }
        },
        Direction::Left => {
            match shape {
                Polyomino::I90Tetromino => mask[0] != 0b1111000, 
                Polyomino::XPentomino => mask[1] != 0b1110000,
                Polyomino::JPentomino => mask[2] != 0b1110000,
                Polyomino::ITetromino => mask[0] != 0b1000000,
                Polyomino::OTetromino => mask[0] != 0b1100000
            }
        },
    }
}


fn can_move(slice: &[u8], mask: &Vec<u8>) -> bool {
    let mask: Vec<u8> = mask.iter().map(|b| b.clone()).rev().collect();
    println!("    Testing move");
    println!("    mask");
    print_bits(&mask);
    println!("    slice");
    print_slice(&slice);
    for (i, line) in slice.iter().rev().enumerate() {
        if line & mask[i] != 0 {
            return false
        }
    }
    return true;
}


fn test_move(board: &Vec<u8>, depth: usize, rock_height: usize, mask: &Vec<u8>) -> bool {
    let range = if depth < rock_height {
        board.len()-depth..board.len()
    } else {
        board.len()-depth..board.len()-depth+rock_height
    };
    let current_slice = &board[range];

    can_move(current_slice, &mask) 
}

fn print_bits(bits: &Vec<u8>) -> () {
    for l in bits.iter() {
        println!("{:#09b}", l);
    }
}

fn print_slice(bits: &[u8]) -> () {
    for l in bits.iter() {
        println!("{:#09b}", l);
    }
}
