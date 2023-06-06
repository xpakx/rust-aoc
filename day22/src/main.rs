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
    let height = second_star(&map, &instructions, &start);
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

fn second_star(map: &Vec<Vec<Tile>>, _instructions: &Vec<Move>, _start: &(usize, usize)) -> usize {
    construct_cubes(map);
    0
}

fn construct_cubes(map: &Vec<Vec<Tile>>) -> () {
    let mut cube = Cube::from(&map);
    for i in cube.sides.iter() {
        for j in i.iter() {
            match j {
                Plane::Empty => print!("."),
                Plane::Board(_) => print!("#"),
            }
        }
    println!("");
    }
    for i in cube.sides.iter() {
        for j in i.iter() {
            match j.rotated(-1) {
                Plane::Empty => {},
                Plane::Board(plane) => {
                    for a in plane.iter() {
                        for b in a.iter() {
                            print!("{:?}", b.tile);
                        }
                        println!("");
                    }
                },
            }
        }
    println!("");
    }


}

#[derive(Debug)]
struct Cube {
    sides: Vec<Vec<Plane>>
}

impl Cube {
    pub fn from(map: &Vec<Vec<Tile>>) -> Self {
        let max_row_len = map
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap();
        let column_len = map.len();
        let (columns, rows, size) = match (column_len, max_row_len) {
            (c, r) if c/4 == r/3 => (4, 3, c/4),
            (c, r) if c/3 == r/4 => (3, 4, c/3),
            (c, r) if c/5 == r/2 => (5, 3, c/5),
            (c, r) if c/2 == r/5 => (2, 5, c/2),
            (_,_) => panic!(""),
        };
        let mut sides = Vec::new();
        for i in 0..columns {
            let mut new_row = Vec::new();
            for j in 0..rows {
               if size*i >= map.len() || size*j >= map[size*i].len() {
                   new_row.push(Plane::Empty);
               } else if let Tile::Nothing = map[size*i][size*j]  {
                   new_row.push(Plane::Empty);
               } else {
                   let mut plane = Vec::new();
                   for ip in size*i..(size*i+size) {
                       let mut plane_row = Vec::new();
                       for jp in size*j..(size*j+size) {
                           plane_row.push(PlaneTile {
                               tile: map[ip][jp].clone(),
                               coord: (ip, jp)
                           });
                       }
                       plane.push(plane_row);
                   }
                   new_row.push(Plane::Board(plane));
               }
            }
            sides.push(new_row);
        }
        return Cube {sides}
    }
}

#[derive(Debug)]
enum Plane {
    Empty,
    Board(Vec<Vec<PlaneTile>>)
}

impl Plane {
    fn rotated(&self, n_clockwise: isize) -> Plane {
        match self {
            Self::Empty => Plane::Empty,
            Self::Board(tiles) => {
                match (n_clockwise).rem_euclid(4) {
                    0 => Plane::Board(tiles.iter().cloned().collect()),
                    1 => { 
                        let mut new_plane = Vec::new();
                        for i in 0..tiles[0].len() {
                            let row: Vec<PlaneTile> = tiles[..].iter().map(|it| it[i].clone()).rev().collect();
                            new_plane.push(row);
                        }
                        Plane::Board(new_plane)
                    },
                    2 => {
                        let mut new_plane = Vec::new();
                        for i in (0..tiles.len()).rev() {
                            let row: Vec<PlaneTile> = tiles[i].iter().map(|a| a.clone()).rev().collect();
                            new_plane.push(row);
                        }
                        Plane::Board(new_plane)
                    },
                    3 => {
                        let mut new_plane = Vec::new();
                        for i in (0..tiles[0].len()).rev() {
                            let row: Vec<PlaneTile> = tiles[..].iter().map(|it| it[i].clone()).collect();
                            new_plane.push(row);
                        }
                        Plane::Board(new_plane)
                    },
                    _ => Plane::Empty,
                }
            }
        }
        
    }
}

#[derive(Debug, Clone)]
struct PlaneTile {
    tile: Tile,
    coord: (usize, usize)
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

#[derive(Clone, Debug)]
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
