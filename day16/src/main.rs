use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut valves = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match parse_line(&line) {
            Some(valve) => { valves.push(valve) },
            None => {}
        };
    }

    first_star(&valves);
    second_star(&valves);

}

fn second_star(valves: &Vec<Valve>) -> () {
}

fn first_star(valves: &Vec<Valve>) -> () {
    let mut time_left = 30;
    while time_left > 0 {
        time_left -= 1;
    }
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

fn parse_line(line: &String) -> Option<Valve> {
    let re = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=([0-9]+); tunnel[s]? lead[s]? to valve[s]? ([A-Z ,]+)").unwrap();
    let parsed = re.captures(line)?;
    let name = parsed.get(1).map_or(String::from(""), |a| a.as_str().to_string());
    let flow = parsed.get(2).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let tunnels = parsed.get(3).map_or("", |a| a.as_str());
    let parsed_tunnels: Vec<Tunnel> = tunnels
        .split(", ")
        .map(str::to_string)
        .map(|s| Tunnel { valve: s, cost: 1 })
        .collect();
    Some(Valve { id: name, flow_rate: flow, tunnels: parsed_tunnels })
}

struct Valve {
    id: String,
    flow_rate: usize,
    tunnels: Vec<Tunnel>
}

#[derive(Debug)]
struct Tunnel {
    cost: usize,
    valve: String
}
