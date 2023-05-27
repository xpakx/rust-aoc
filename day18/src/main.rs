use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut vortices = HashSet::new();
    while let Some(Ok(line)) = lines.next() {
        match parse_line(&line) {
            Some(coord) => { vortices.insert(coord);},
            None => {}
        };
    }

    first_star(&vortices);
    second_star(&vortices);

}

fn second_star(valves: &HashSet<Coord>) -> () {
}

fn first_star(valves: &HashSet<Coord>) -> () {
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

fn parse_line(line: &String) -> Option<Coord> {
    let mut splitted = line.split(",");
    let x = match splitted.next() {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(_) => return None
        },
        None => return None
    };
    let y = match splitted.next() {
        Some(y) => match y.parse() {
            Ok(y) => y,
            Err(_) => return None
        },
        None => return None
    };
    let z = match splitted.next() {
        Some(z) => match z.parse() {
            Ok(z) => z,
            Err(_) => return None
        },
        None => return None
    };
    Some(Coord {x, y, z})
}

#[derive(Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}
