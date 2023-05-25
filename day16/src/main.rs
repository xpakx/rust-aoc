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
    let mut distances = HashMap::new();
    for valve in valves.iter() {
        distances.insert(valve.id.clone(), dijkstra(valves, &valve.id));
    }
    let potential_valves: Vec<(String, usize)> = valves
        .iter()
        .filter(|v| v.flow_rate != 0)
        .map(|v| (v.id.clone(), v.flow_rate))
        .collect();
    let result = choose_valve(&distances, 30, &String::from("AA"), potential_valves);
    println!("{}", result);
}

fn choose_valve(distance_map: &HashMap<String, HashMap<String, usize>>, time_left: usize, current_valve: &String, potential_valves: Vec<(String, usize)>) -> usize {
    if time_left == 0 {
        return 0
    }
    let distances = distance_map.get(current_valve).unwrap();
    let mut max_flow = 0;

    for valve in potential_valves.iter() {
        let new_valves: Vec<(String, usize)> = potential_valves
            .iter()
            .filter(|a| a.0 != valve.0)
            .map(|a| (a.0.clone(), a.1))
            .collect();
        let distance = distances.get(&valve.0).unwrap(); 
        let new_time = if time_left < distance + 1 {0} else {time_left - distance - 1};
        let flow =  new_time * valve.1;
        let total_flow = flow + choose_valve(distance_map, new_time, &valve.0, new_valves);
        if total_flow > max_flow {
            max_flow = total_flow;
        }
    }
    max_flow
}

fn dijkstra(valves: &Vec<Valve>, current_valve: &String) -> HashMap<String, usize> {
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

    let result = valves
        .iter()
        .map(|v| v.id.clone())
        .map(|id| {
            let distance = *dist.get(&id).unwrap();
            (id, distance)
        })
    .collect();
    result
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
