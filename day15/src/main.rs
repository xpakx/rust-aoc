use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut sensors: Vec<Sensor> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match parse_line(&line) {
            Some(sensor) => sensors.push(sensor),
            None => {}
        };
    }

    let line = 2000000;
    let points_info = sensors
        .iter()
        .map(|s| ((line-s.localization.y).abs() as u32, s))
        .filter(|s| s.0 <= s.1.distance)
        .map(|s| ((s.1.distance - s.0) as i32, s.1.localization.x));
    let mut points = HashSet::new();
    for point in points_info {
        points.insert(point.1);
        for i in 1..point.0+1 {
            points.insert(point.1+i);
            points.insert(point.1-i);
        }
    }
    let mut beacons: Vec<i32> = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.y == line)
        .map(|b| b.x)
        .filter(|b| points.contains(b))
        .collect();
    beacons.sort_unstable();
    beacons.dedup();

    let beacons_count = beacons.iter().count();

    println!("Result: {}", points.len() - beacons_count);

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

fn parse_line(line: &String) -> Option<Sensor> {
    let re = Regex::new(r"Sensor at x=([-]?[0-9]+), y=([-]?[0-9]+): closest beacon is at x=([-]?[0-9]+), y=([-]?[0-9]+)").unwrap();
    let parsed = re.captures(line)?;
    let x1 = parsed.get(1).map_or(0, |a| a.as_str().parse::<i32>().unwrap());
    let y1 = parsed.get(2).map_or(0, |a| a.as_str().parse::<i32>().unwrap());
    let x2 = parsed.get(3).map_or(0, |a| a.as_str().parse::<i32>().unwrap());
    let y2 = parsed.get(4).map_or(0, |a| a.as_str().parse::<i32>().unwrap());
    
    let dist: u32 = ((x1-x2).abs() + (y1-y2).abs()) as u32;

    Some(Sensor { localization: Position {x: x1, y: y1}, beacon: Position {x: x2, y: y2}, distance: dist })
}

struct Sensor {
    localization: Position,
    beacon: Position,
    distance: u32
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32
}
