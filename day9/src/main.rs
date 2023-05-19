use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    move_rope(1);
    move_rope(9);
}

fn move_rope(knots_num: usize) {
    let lines = read_input().expect("Should read from file");
    let mut head = Node { position: (0, 0) };
    let mut knots = Vec::new();
    for _ in 0..knots_num {
        knots.push(Node { position: (0, 0) });
    }
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert((0, 0));

    for line in lines {
        if let Ok(line) = line {
            let instruction = parse_line(&line)
                .expect("Should parse instruction");
            for _ in 0..instruction.1 {
                head.update_pos(to_vector(&instruction.0));
                knots[0].follow(&head);
                for i in 1..knots_num {
                    let last = knots[i-1];
                    knots[i].follow(&last);
                }
                visited.insert(knots.last().unwrap().position);
            }
        }
    }
    println!("Visited: {}", visited.len());
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn read_input() -> Result<Lines<BufReader<File>>, io::Error> {
    let file_path = Path::new("./input.txt");
    let file =  match File::open(&file_path){
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(io::BufReader::new(file).lines());
}

fn parse_line(line: &String) -> Option<(Direction, u32)> {
    let mut parsed = line.split(" ");
    let dir = match parsed.next() {
        Some(p) => match p {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return None
        },
        None => return None
    };
    let num = match parsed.next() {
        Some(n) => match n.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return None
        },
        None => return None
    };
    return Some((dir, num));
}

#[derive(Clone, Copy)]
struct Node {
    position: (i32, i32)
}

impl Node {
    fn update_pos(&mut self, vec: (i32, i32)) {
        self.position.0 += vec.0;
        self.position.1 += vec.1;
    }

    fn follow(&mut self, other: &Node) {
        let diff_x = (self.position.0 - other.position.0).abs();
        let diff_y = (self.position.1 - other.position.1).abs();
        if diff_x < 2 && diff_y < 2 {
            return;
        }
        let mut move_vec = (
            other.position.0 - self.position.0,
            other.position.1 - self.position.1
            );
        if diff_x == 2 {
            move_vec.0 = (other.position.0 - self.position.0)/2;
        }
        if diff_y == 2 {
            move_vec.1 = (other.position.1 - self.position.1)/2;
        }
        self.update_pos(move_vec);
    }
}

fn to_vector(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Down => (0, -1),
        Direction::Up => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0)
    }
}
