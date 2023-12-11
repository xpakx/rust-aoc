use std::fs;
use std::path::Path;
use std::io;
use std::collections::HashMap;
use std::cmp::min;

fn main() {
    let height = first_star(2022);
    println!("Height: {}", height);
    let height = second_star(1000000000000);
    println!("Height: {}", height);
}

fn first_star(rocks: usize) -> usize {
    let input = read_input().expect("Should read input from file");
    let directions: Vec<Direction> = parse_input(&input);
    let dir_len = directions.len();
    let mut board = vec![0b1111111];
    let mut dir = 0;
    let mut total_height = 0;
    for i in 0..rocks {
        let rock = get_polyomino(i);
        let mut mask = polyomino_to_bit_mask(&rock);
        let rock_height = mask.len();
        let mut stopped = false;
        let mut depth = 0;
        let mut free_fall = 3;

        while !stopped {
            let direction = directions[dir%dir_len];
            if no_wall(&mask, &direction, &rock) {
                let new_mask = move_horizontal(&direction, &mask);
                if depth == 0 || test_move(&board, depth, rock_height, &new_mask) {
                    mask = new_mask;
                } 
            }

            dir+= 1;

            if free_fall == 0 {
                // falling
                depth += 1;
                if !test_move(&board, depth, rock_height, &mask) {
                    depth -= 1;
                    let height = board.len();
                    let mask: Vec<u8> = mask.iter().map(|b| b.clone()).rev().collect();
                    for i in 0..min(depth, rock_height) {
                        board[height-depth+i] = board[height-depth+i] | mask[i];
                    }
                    for i in depth..rock_height {
                        board.push(mask[i]);
                    }
                    stopped = true;
                    if depth < rock_height {
                        total_height += rock_height - depth;
                    }
                }
            } else {
                free_fall -= 1;
            }
        }
        board = simplify_board(&board);
    }
    total_height
}

fn second_star(rocks: usize) -> usize {
    let input = read_input().expect("Should read input from file");
    let directions: Vec<Direction> = parse_input(&input);
    let dir_len = directions.len();
    let mut board = vec![0b1111111];
    let mut dir = 0;
    let mut total_height = 0;
    let mut heights = Vec::new();
    let mut state_map = HashMap::new();
    let mut cycle_end = 0;
    let mut cycle_start = 0;
    for i in 0..10000 {
        let rock = get_polyomino(i);
        let mut mask = polyomino_to_bit_mask(&rock);
        let rock_height = mask.len();
        let mut stopped = false;
        let mut depth = 0;
        let mut free_fall = 3;

        while !stopped {
            let direction = directions[dir%dir_len];
            if no_wall(&mask, &direction, &rock) {
                let new_mask = move_horizontal(&direction, &mask);
                if depth == 0 || test_move(&board, depth, rock_height, &new_mask) {
                    mask = new_mask;
                } 
            }

            dir+= 1;

            if free_fall == 0 {
                // falling
                depth += 1;
                if !test_move(&board, depth, rock_height, &mask) {
                    depth -= 1;
                    let height = board.len();
                    let mask: Vec<u8> = mask.iter().map(|b| b.clone()).rev().collect();
                    for i in 0..min(depth, rock_height) {
                        board[height-depth+i] = board[height-depth+i] | mask[i];
                    }
                    for i in depth..rock_height {
                        board.push(mask[i]);
                    }
                    stopped = true;
                    if depth < rock_height {
                        total_height += rock_height - depth;
                    }
                }
            } else {
                free_fall -= 1;
            }
        }
        board = simplify_board(&board);

        heights.push(total_height);
        let hash = generate_hash(&rock, dir%dir_len, &board);
        let elem = state_map.get(&hash);
        if let Some(elem) = elem {
            cycle_end = i;
            cycle_start = *elem;
            break;
        }
        state_map.insert(hash, i);
    }

    let max = rocks; 
    let height_before_cycle = heights[cycle_start];
    let cycle_length = cycle_end - cycle_start;
    let cycle_height = heights[cycle_end] - heights[cycle_start];
    let to_simulate = max - cycle_start - 1;
    
    let full_cycles = to_simulate / cycle_length;
    let after_last_cycle = to_simulate % cycle_length;
    let after_height = if after_last_cycle > 0 {
        heights[cycle_start+after_last_cycle] - heights[cycle_start]
    } else {
        0
    };

    let total_height = height_before_cycle + full_cycles*cycle_height + after_height;

    total_height
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
    for (i, line) in slice.iter().enumerate() {
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

fn move_horizontal(direction: &Direction, mask: &Vec<u8>) -> Vec<u8> {
    if let Direction::Right = direction {
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
    }
}

fn simplify_board(board: &Vec<u8>) -> Vec<u8> {
    let mut new_board: Vec<u8> = Vec::new();
    for i in (0..board.len()).rev() {
        let last = new_board.last();
        if let Some(last) = last {
            if last == &0b1111111 {
                break;
            }
            let top_mask = !last & !board[i];
            let mut left_mask = top_mask.clone();
            let mut pointer = 0b1000000;
            let mut segment = 0b0;
            for _ in 0..7 {
                if pointer & !board[i] == 0 {
                    if segment != 0 && (segment & top_mask != 0) {
                        left_mask = left_mask | segment;                        
                    }
                    segment = 0b0;
                } else {
                    segment = segment | pointer;
                }
                pointer = pointer >> 1;
            }
            let mut right_mask = top_mask.clone();
            let mut pointer = 0b1;
            let mut segment = 0b0;
            for _ in 0..7 {
                if pointer & !board[i] == 0 {
                    if segment != 0 && (segment & top_mask != 0) {
                        right_mask = right_mask | segment;                        
                    }
                    segment = 0b0;
                } else {
                    segment = segment | pointer;
                }
                pointer = pointer << 1;
            }
            let mask = top_mask | left_mask | right_mask;
            let new_line = !(!board[i] & mask);
            new_board.push(new_line);
            
        } else {
            new_board.push(board[i].clone());
        }
    }

    new_board.iter().map(|b| b.clone()).rev().collect()
}

fn generate_hash(rock: &Polyomino, index_dir: usize, board: &Vec<u8>) -> StateKey {
    let poly = match rock {
        Polyomino::I90Tetromino => 0b10000000, 
        Polyomino::XPentomino =>   0b01000000,
        Polyomino::JPentomino =>   0b00100000,
        Polyomino::ITetromino =>   0b00010000,
        Polyomino::OTetromino =>   0b00001000
    };
    let hash: Vec<u8> = board.iter().map(|b| b.clone()).collect();
    StateKey {
        direction: index_dir,
        rock: poly,
        map: hash
    }
}

#[derive(Eq, Hash, PartialEq)]
struct StateKey {
    direction: usize,
    rock: u8,
    map: Vec<u8>
}
