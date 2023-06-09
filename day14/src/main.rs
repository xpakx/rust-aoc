use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    while let Some(Ok(line)) = lines.next() {
        let points = parse_line(&line);
        for i in 0..(points.len()-1) {
            let points_to_add = generate_points(points[i], points[i+1]);
            for point in points_to_add {
                obstacles.insert(point);
            }
        }
    }
    let max = obstacles.iter().map(|a| a.1).max().unwrap_or(0);
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((500,0));
    let mut stationary = 0;
    let mut ground_level = false;
    while !ground_level {
        let mut position = stack.pop().unwrap();
        loop {
            if position.1 == max {
                ground_level = true;
                break;
            }
            let bottom = (position.0, position.1+1);
            let left = (position.0-1, position.1+1);
            let right = (position.0+1, position.1+1);
            if !obstacles.contains(&bottom) {
                stack.push(position);
                position = bottom;
            } else if !obstacles.contains(&left) {
                stack.push(position);
                position = left;
            } else if !obstacles.contains(&right) {
                stack.push(position);
                position = right;
            } else {
                obstacles.insert(position);
                stationary += 1;
                break;
            }
        }
    }
    println!("Units of sand: {}", stationary);
    let ground = max + 2;
    while !obstacles.contains(&(500,0)) {
        let mut position = stack.pop().unwrap();
        loop {
            if position.1 == ground-1 {
                obstacles.insert(position);
                stationary += 1;
                break;
            }
            let bottom = (position.0, position.1+1);
            let left = (position.0-1, position.1+1);
            let right = (position.0+1, position.1+1);
            if !obstacles.contains(&bottom) {
                stack.push(position);
                position = bottom;
            } else if !obstacles.contains(&left) {
                stack.push(position);
                position = left;
            } else if !obstacles.contains(&right) {
                stack.push(position);
                position = right;
            } else {
                obstacles.insert(position);
                stationary += 1;
                break;
            }
        }
    }
    println!("Units of sand: {}", stationary);
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

fn parse_line(line: &String) -> Vec<(usize, usize)> {
    let points = line.split("->")
        .map(|a| a.trim())
        .map(|a| a.split(",").collect())
        .map(|a: Vec<&str>| (a[0], a[1]))
        .map(|(a,b)| (a.parse::<usize>(), b.parse::<usize>()))
        .filter_map(|(a,b)| match (a,b) {
           (Ok(a), Ok(b)) => Some((a,b)),
           (_, _) => None
        })
        .collect();
   return points; 
}

fn generate_points(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let first_coord_const = start.0 == end.0;
    let gen = match first_coord_const {
        true => match (start.1, end.1) {
            (s,e) if s < e => PointGeneration { stable: start.0, start: s, end: e},
            (s,e) => PointGeneration { stable: start.0, start: e, end: s}
        },
        false => match (start.0, end.0) {
            (s,e) if s < e => PointGeneration { stable: start.1, start: s, end: e},
            (s,e) => PointGeneration { stable: start.1, start: e, end: s}
        }
    };
    let mut points = Vec::new();

    for i in gen.start..(gen.end+1) {
       if first_coord_const {
           points.push((gen.stable, i));
       } else {
           points.push((i, gen.stable));
       }
    };

    points
}

struct PointGeneration {
    stable: usize,
    start: usize,
    end: usize
}
