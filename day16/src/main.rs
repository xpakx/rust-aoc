use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use regex::Regex;
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

    first_star(&mut valves);
    second_star();

}

fn second_star() -> () {
    println!("Not implemented yet");
}

fn first_star(valves: &mut Vec<Valve>) -> () {
    let mut time_left = 30;
    let mut current_valve = String::from("AA");
    while time_left > 0 {
        let max = dijkstra(valves, time_left, &current_valve);
        println!("Next: {}, cost: {}, pressure: {}", max.0, max.2, max.1);
        time_left -= max.2;
        current_valve = max.0;
        if max.1 == 0 {
            break;
        }
        println!("Time left: {}", time_left);
        println!("Turn {}", 31-time_left);
        for v in valves.iter_mut() {
            if v.id == current_valve {
                v.flow_rate = 0;
                break;
            }
        }
    }
}

fn dijkstra(valves: &Vec<Valve>, time_left: usize, current_valve: &String) -> (String, usize, usize) {
    let mut dist: HashMap<String, usize> = HashMap::new();
    let mut q: Vec<&Valve> = Vec::new();
    for valve in valves.iter() {
            if valve.id == *current_valve { 
                dist.insert(valve.id.clone(), 0);
            } else {
                dist.insert(valve.id.clone(), usize::MAX);
            }
            q.push(valve);
    }

    while !q.is_empty() {
        let x = q.iter().min_by_key(|a| dist.get(&a.id).unwrap_or(&usize::MAX)).unwrap();
        let point = x.clone();
        q = q.iter().filter(|&a| a.id != x.id).map(|a| a.clone()).collect();
        let current = dist.get(&point.id).unwrap() + 0;

        for tunnel in point.tunnels.iter() {
            let n_dist = dist.get(&tunnel.valve).unwrap();
            let alt = current + tunnel.cost;
            if alt < *n_dist {
                dist.insert(tunnel.valve.clone(), alt);
            }
        }
    }

    valves
        .iter()
        .map(|v| (v.id.clone(), v.flow_rate))
        .map(|(id, flow)| {
            let distance = *dist.get(&id).unwrap();
            let total_flow = if time_left <= distance + 2 {0} else {(time_left - distance - 2) * flow};
            println!("candidate {}: flow: {}, distance: {}", id, total_flow, distance);
            (id.clone(), total_flow, distance)
        })
        .max_by_key(|a| a.1)
        .unwrap()
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
