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
                println!("{:?}", monkey);
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

    0
}

fn second_star(monkeys: &HashMap<String, Monkey>) -> usize {
    0
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
            first_number: None,
            second_number: None,
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
            first_number: None,
            second_number: None,
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
    first_number: Option<usize>,
    second_number: Option<usize>,
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
    Division,
    Subtraction
}
