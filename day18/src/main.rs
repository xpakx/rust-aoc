use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut cubes = HashSet::new();
    while let Some(Ok(line)) = lines.next() {
        match parse_line(&line) {
            Some(coord) => { cubes.insert(coord);},
            None => {}
        };
    }

    let first = first_star(&cubes);
    println!("First star: {}", first);
    second_star(&cubes);

}

fn first_star(cubes: &HashSet<Coord>) -> usize {
    cubes
        .iter()
        .map(|cube| {
            neighbours(&cube)
                .iter()
                .filter(|n| !cubes.contains(n))
                .count()
        })
        .sum()
}

fn second_star(cubes: &HashSet<Coord>) -> () {
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
    x: i32,
    y: i32,
    z: i32,
}

fn neighbours(coord: &Coord) -> Vec<Coord> {
    let mut result = Vec::new();
    result.push(Coord {x: coord.x - 1, y: coord.y, z: coord.z});
    result.push(Coord {x: coord.x + 1, y: coord.y, z: coord.z});
    result.push(Coord {x: coord.x, y: coord.y - 1, z: coord.z});
    result.push(Coord {x: coord.x, y: coord.y + 1, z: coord.z});
    result.push(Coord {x: coord.x, y: coord.y, z: coord.z - 1});
    result.push(Coord {x: coord.x, y: coord.y, z: coord.z + 1});
    result
}
