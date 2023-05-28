use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut blueprints = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        match parse_line(&line) {
            Some(blueprint) => { blueprints.push(blueprint);},
            None => {}
        };
    }

    let first = first_star(&blueprints);
    println!("First star: {}", first);
    let second = second_star(&blueprints);
    println!("Second star: {}", second);

}

fn first_star(blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .iter()
        .map(|b| (b.id, find_max_geodes_production(b, 24)))
        .map(|(b, g)| b*g)
        .sum()
}

fn find_max_geodes_production(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut states = Vec::new();
    let mut visited = HashSet::new();
    let mut max_geodes = 0;
    let max_ore_cost = vec![blueprint.ore_bot_cost, blueprint.clay_bot_cost, blueprint.obsidian_bot_cost, blueprint.geode_bot_cost]
        .into_iter()
        .max()
        .unwrap_or(0);
    states.push(State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0
    });
    for i in 0..minutes {
        let mut new_states = Vec::new();
        while let Some(state) = states.pop() {
            if blueprint.ore_bot_cost <= state.ore && state.ore_bots <= max_ore_cost {
                let new_state = generate_new_state(&blueprint, &state, &BotType::Ore);
                let max_potential_geodes = new_state.geode + (minutes-i)*(new_state.geode_bots+minutes-i);
                if !visited.contains(&new_state) && max_potential_geodes > max_geodes {
                    new_states.push(new_state);
                    visited.insert(new_state);
                    if new_state.geode > max_geodes {
                        max_geodes = new_state.geode;
                    }
                }
            }

            if blueprint.clay_bot_cost <= state.ore && state.clay_bots <= blueprint.obsidian_bot_clay_cost {
                let new_state = generate_new_state(&blueprint, &state, &BotType::Clay);
                let max_potential_geodes = new_state.geode + (minutes-i)*(new_state.geode_bots+minutes-i);
                if !visited.contains(&new_state) && max_potential_geodes > max_geodes {
                    new_states.push(new_state);
                    visited.insert(new_state);
                    if new_state.geode > max_geodes {
                        max_geodes = new_state.geode;
                    }
                }
            }

            if blueprint.obsidian_bot_cost <= state.ore && blueprint.obsidian_bot_clay_cost <= state.clay && state.obsidian_bots <= blueprint.geode_bot_obsidian_cost  {
                let new_state = generate_new_state(&blueprint, &state, &BotType::Obsidian);
                let max_potential_geodes = new_state.geode + (minutes-i)*(new_state.geode_bots+minutes-i);
                if !visited.contains(&new_state) && max_potential_geodes > max_geodes {
                    new_states.push(new_state);
                    visited.insert(new_state);
                    if new_state.geode > max_geodes {
                        max_geodes = new_state.geode;
                    }
                }
            }

            if blueprint.geode_bot_cost <= state.ore  && blueprint.geode_bot_obsidian_cost <= state.obsidian {
                let new_state = generate_new_state(&blueprint, &state, &BotType::Geode);
                if !visited.contains(&new_state) {
                    new_states.push(new_state);
                    visited.insert(new_state);
                    if new_state.geode > max_geodes {
                        max_geodes = new_state.geode;
                    }
                }
            }

            if state.ore <= max_ore_cost && state.ore_bots <= max_ore_cost {
                new_states.push(State {
                    ore: state.ore + state.ore_bots,
                    clay: state.clay + state.clay_bots,
                    obsidian: state.obsidian + state.obsidian_bots,
                    geode: state.geode + state.geode_bots,
                    ore_bots: state.ore_bots,
                    clay_bots: state.clay_bots,
                    obsidian_bots: state.obsidian_bots,
                    geode_bots: state.geode_bots
                });
                let geode = state.geode + state.geode_bots;
                if geode > max_geodes {
                    max_geodes = geode;
                }
            }
        }
        states = new_states;
    }
    max_geodes
}

fn second_star(blueprints: &Vec<Blueprint>) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|b| find_max_geodes_production(b, 32))
        .product()
}

enum BotType {
    Ore, Clay, Obsidian, Geode
}

fn generate_new_state(blueprint: &Blueprint, state: &State, bot: &BotType) -> State {
    let ore_cost = match bot {
        BotType::Ore => blueprint.ore_bot_cost,
        BotType::Clay => blueprint.clay_bot_cost,
        BotType::Obsidian => blueprint.obsidian_bot_cost,
        BotType::Geode => blueprint.geode_bot_cost,
    };
    let clay_cost = match bot {
        BotType::Obsidian => blueprint.obsidian_bot_clay_cost,
        _ => 0
    };
    let obsidian_cost = match bot {
        BotType::Geode => blueprint.geode_bot_obsidian_cost,
        _ => 0
    };
    let ore = state.ore + state.ore_bots - ore_cost;
    let clay = state.clay + state.clay_bots - clay_cost;
    let obsidian = state.obsidian + state.obsidian_bots - obsidian_cost;
    let geode = state.geode + state.geode_bots;
    let ore_bots = state.ore_bots + match bot { BotType::Ore => 1, _ => 0};
    let clay_bots = state.clay_bots + match bot { BotType::Clay => 1, _ => 0};
    let obsidian_bots = state.obsidian_bots + match bot { BotType::Obsidian => 1, _ => 0};
    let geode_bots = state.geode_bots + match bot { BotType::Geode => 1, _ => 0};

    State {ore, clay, obsidian, geode, ore_bots, clay_bots, obsidian_bots, geode_bots}
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

fn parse_line(line: &String) -> Option<Blueprint> {
    let re = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
    let parsed = re.captures(line)?;
    let id = parsed.get(1).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let ore_bot_cost = parsed.get(2).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let clay_bot_cost = parsed.get(3).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let obsidian_bot_cost = parsed.get(4).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let obsidian_bot_clay_cost = parsed.get(5).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let geode_bot_cost = parsed.get(6).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    let geode_bot_obsidian_cost = parsed.get(7).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
    Some(Blueprint { 
        id, 
        ore_bot_cost, 
        clay_bot_cost, 
        obsidian_bot_cost,
        obsidian_bot_clay_cost,
        geode_bot_cost,
        geode_bot_obsidian_cost 
    })
}

#[derive(Clone, Copy, Debug)]
struct Blueprint {
    id: usize,
    ore_bot_cost: usize,
    clay_bot_cost: usize,
    obsidian_bot_cost: usize,
    obsidian_bot_clay_cost: usize,
    geode_bot_cost: usize,
    geode_bot_obsidian_cost: usize
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
struct State {
    ore: usize,
    clay: usize, 
    obsidian: usize,
    geode: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize
}
