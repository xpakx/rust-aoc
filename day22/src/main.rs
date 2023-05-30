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
    let map2: Vec<Vec<Tile>> = append_empty_tiles(&map);
    let start: (usize, usize) = get_starting_position(&map[0]).expect("First row should contain open position");
    let height = first_star(&map2, &instructions, &start);
    println!("First star: {}", height);
    let height = second_star();
    println!("Second star: {}", height);
}

fn first_star(map: &Vec<Vec<Tile>>, instructions: &Vec<Move>, start: &(usize, usize)) -> usize {
    use Direction::*;
    let mut position = start.clone();
    let mut direction = Right;
    for inst in instructions.iter() {
        match inst {
            Move::Rotate(rotation) => {
                direction = direction.rotate(&rotation);
            },
            Move::Forward(distance) => {
                position = find_achievable_position(&map, &position, distance, &direction);
            },
        }
    }
    
    return (1 + position.0) * 1000 + (1 + position.1) * 4 + direction.value()
}

fn second_star() -> usize {
    0
}

fn find_achievable_position(map: &Vec<Vec<Tile>>, start: &(usize, usize), steps: &u32, direction: &Direction) -> (usize, usize) {
    let mut candidate = start.clone();
    for _ in 0..steps.clone() {
        let mut pos = next_pos(&candidate, direction, map);
        while let Tile::Nothing = map[pos.0][pos.1] {
            pos = next_pos(&pos, direction, map);
        }
        match map[pos.0][pos.1] {
            Tile::Wall => return candidate,
            Tile::Nothing => {},
            Tile::Floor => candidate = pos,
        }
    }

    candidate
}

fn next_pos(position: &(usize, usize), direction: &Direction, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    use Direction::*;
    match direction {
        Down => ((position.0 + 1).rem_euclid(map.len()), position.1.clone()),
        Up => ((position.0 as i32 - 1).rem_euclid(map.len() as i32) as usize, position.1.clone()),
        Left => (position.0.clone(), (position.1 as i32 - 1).rem_euclid(map[position.0].len() as i32) as usize),
        Right => (position.0.clone(), (position.1 + 1).rem_euclid(map[position.0].len())),
    }
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

fn append_empty_tiles(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut max_length = 0;
    for row in map.iter() {
        if row.len() > max_length {
            max_length = row.len();
        }
    }
    map
        .iter()
        .map(|row| {
            let mut new_row: Vec<Tile> = row
                .iter()
                .map(|t| t.clone())
                .collect();
            if row.len() < max_length {
                new_row.extend(vec![Tile::Nothing; max_length - row.len()]);
            }
            new_row
        })
    .collect()
}

#[derive(Clone)]
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> usize {
        use Direction::*;
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }

    fn rotate(self, rotation: &Rotation) -> Direction {
        use Direction::*;
        match self {
            Right => match rotation {
                Rotation::Left => Up,
                Rotation::Right => Down,
            },
            Down => match rotation {
                Rotation::Left => Right,
                Rotation::Right => Left,
            },
            Left => match rotation {
                Rotation::Left => Down,
                Rotation::Right => Up,
            },
            Up => match rotation {
                Rotation::Left => Left,
                Rotation::Right => Right,
            },
        }
    }
}

fn get_starting_position(row: &Vec<Tile>) -> Option<(usize, usize)> {
    for i in 0..row.len() {
        if let Tile::Floor = row[i] {
            return Some((0, i))
        }
    }

    None
}
