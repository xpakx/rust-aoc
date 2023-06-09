use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut monkeys = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        match parse_line(&line) {
            Some(monkey) => { 
                monkeys.insert(monkey.id.clone(), monkey);
            },
            None => {}
        };
    }

    let first = first_star(&monkeys);
    println!("First star: {}", first);
    let second = second_star(&monkeys);
    println!("Second star: {}", second);

}

fn first_star(monkeys: &HashMap<String, Monkey>) -> usize {
    calculate(&String::from("root"), &monkeys)
}


fn calculate(monkey_id: &String, monkeys: &HashMap<String, Monkey>) -> usize {
    let monkey = monkeys.get(monkey_id).unwrap();
    if let Some(num) = monkey.number {
        return num;
    }
    let num1 = if let Some(child) = monkey.first_child.clone() {
        calculate(&child, &monkeys)
    } else {0};
    let num2 = if let Some(child) = monkey.second_child.clone() {
        calculate(&child, &monkeys)
    } else {0};
    match monkey.operation {
        Some(o) => match o {
            Operation::Addition => num1 + num2,
            Operation::Multiplication => num1 * num2,
            Operation::Subtraction => num1 - num2,
            Operation::Division => num1 / num2
        },
        None => 0
    }
}

fn get_human_path(monkey_id: &String, monkeys: &HashMap<String, Monkey>) -> Option<Vec<String>> {
    if monkey_id == &"humn" {
        return Some(vec![String::from("humn")]);
    }
    let monkey = monkeys.get(monkey_id).unwrap();
    if let Some(_) = monkey.number {
        return None
    }
    if let Some(child) = monkey.first_child.clone() {
        if let Some(mut result) = get_human_path(&child, &monkeys) {
            result.insert(0, monkey_id.clone());
            return Some(result);
        }
    }
    if let Some(child) = monkey.second_child.clone() {
        if let Some(mut result) = get_human_path(&child, &monkeys) {
            result.insert(0, monkey_id.clone());
            return Some(result);
        }
    }
    None
}

fn second_star(monkeys: &HashMap<String, Monkey>) -> usize {
    let human = get_human_path(&String::from("root"), &monkeys).unwrap();
    let mut target = 0;
    for i in 0..human.len()-1 {
        let id = human[i].clone();
        let id_next = human[i+1].clone();
        let current = monkeys.get(&String::from(&id)).unwrap();
        let first_child = current.first_child.clone().unwrap();
        let second_child = current.second_child.clone().unwrap();
        let first = first_child == id_next;
        let other_child = match first {
            false => first_child,
            true => second_child
        };
        let other = calculate(&other_child, &monkeys);
        if id == "root" {
            target = other;
        } else {
            let operation = current.operation.clone().unwrap();
            match operation {
                Operation::Addition => target = target - other,
                Operation::Multiplication => target = target / other,
                Operation::Subtraction => match first {
                    true => target = target + other,
                    false => target = other - target
                },
                Operation::Division => match first {
                    true => target = target * other,
                    false => target = other / target
                }
            };
        }
    }

    target
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

fn parse_line(line: &String) -> Option<Monkey> {
    let re = Regex::new(r"([a-z]+): ([a-z]+) ([\*\+-/]) ([a-z]+)").unwrap();
    if let Some(line) = re.captures(line) {
        let id = String::from(line.get(1).map_or("", |a| a.as_str()));
        let id_first = String::from(line.get(2).map_or("", |a| a.as_str()));
        let id_second = String::from(line.get(4).map_or("", |a| a.as_str()));
        let operation = match line.get(3).map_or("", |a| a.as_str()) {
            "*" => Operation::Multiplication,
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            _ => Operation::Division,
        };
        return Some(Monkey {
            id: id,
            first_child: Some(id_first),
            second_child: Some(id_second),
            operation: Some(operation),
            number: None,
        });
    }
    let re= Regex::new(r"([a-z]+): ([0-9]+)").unwrap();
    if let Some(line) = re.captures(line) {
        let id = String::from(line.get(1).map_or("", |a| a.as_str()));
        let number = line.get(2).map_or(0, |a| a.as_str().parse::<usize>().unwrap());
        return Some(Monkey {
            id: id,
            first_child: None,
            second_child: None,
            operation: None,
            number: Some(number),
        });
    }
    None
}

#[derive(Debug)]
struct Monkey {
    id: String,
    number: Option<usize>,
    operation: Option<Operation>,
    first_child: Option<String>,
    second_child: Option<String>,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Addition,
    Multiplication,
    Division,
    Subtraction
}
