use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    first_star();
}

fn first_star() {
    let mut id = 0;
    let mut items = Vec::new();
    let mut test = 0;
    let mut id_false = 0;
    let mut id_true = 0;
    let mut operation = None;
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines = read_input().expect("Should read from file");
    for line in lines {
        if let Ok(line) = line {
            let instruction = parse_line(&line);
            if instruction.is_none() {
                let monkey = Monkey { 
                    id, 
                    items: items.to_vec(), 
                    test, 
                    id_false, id_true, 
                    operation: operation.unwrap()
                };
                println!("{:?}", monkey);
                monkeys.push(monkey);
                continue;
            }
            let instruction = instruction.unwrap(); 

            match instruction.0 {
                Instruction::MonkeyDef => {
                    id = instruction.1.unwrap_or(0);
                },
                Instruction::Items => {
                    items = instruction.2.as_ref().unwrap().clone();
                },
                Instruction::Test => {
                    test = instruction.1.unwrap_or(0);
                },
                Instruction::TestTrue => {
                    id_true = instruction.1.unwrap_or(0);
                },
                Instruction::TestFalse => {
                    id_false = instruction.1.unwrap_or(0);
                },
                Instruction::Operation => {
                    operation = instruction.3;
                }
            };
            println!("{:?}", instruction);
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: Vec<u32>,
    test: u32,
    id_true: u32,
    id_false: u32,
    operation: Operation
}


#[derive(Debug)]
enum Instruction {
    MonkeyDef,
    Items,
    Operation,
    Test,
    TestTrue,
    TestFalse,
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    operation: OperationType,
    first: OperationElem,
    second: OperationElem
}

#[derive(Debug, Copy, Clone)]
enum OperationType {
    Add,
    Multiply
}

#[derive(Debug, Copy, Clone)]
enum OperationElem {
    Old,
    Number(u32)
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

fn parse_line(line: &String) -> Option<(Instruction, Option<u32>, Option<Vec<u32>>, Option<Operation>)> {
    let mut parsed = line.trim().split(" ");
    let action = match parsed.next() {
        Some(p) => match p {
            "Monkey" => Instruction::MonkeyDef,
            "Starting" => Instruction::Items,
            "Operation:" => Instruction::Operation,
            "Test:" => Instruction::Test,
            "If" => match parsed.next() {
                Some(a) => match a {
                    "true:" => Instruction::TestTrue,
                    "false:" => Instruction::TestFalse,
                    _ => return None
                },
                None => return None
            }
            _ => return None
        },
        None => return None
    };
    let num = match action {
        Instruction::Operation | Instruction::Items => None,
        Instruction::MonkeyDef => match parsed.last() {
            Some(a) => a.strip_suffix(":"),
            None => None
        },
        _ => match parsed.last() {
            Some(a) => Some(a),
            None => None
        }
    };
    let num = match num {
        Some(n) => match n.parse::<u32>() {
            Ok(n) => Some(n),
            Err(_) => None
        },
        None => None
    };
    let list = match action {
        Instruction::Items => {
            let mut parsed_items = line.trim().split(":");
            let item_string = parsed_items.nth(1).expect("Should have items");
            let mut items = Vec::new();
            for item in item_string.split(", ") {
                let item_num = item.trim().parse();
                if item_num.is_ok() {
                    items.push(item_num.unwrap());
                }
            }
            Some(items)
        },
        _ => None
    };
    let operation = match action {
        Instruction::Operation => {
            let parsed_op = line.trim().split("=");
            let op = parsed_op.last().unwrap().trim();
            let mut operation_elems = op.split(" ");
            let first = match operation_elems.next().unwrap() {
                "old" => OperationElem::Old,
                x => OperationElem::Number(x.parse().unwrap())
            };
            let symbol = match operation_elems.next().unwrap() {
                "+" => OperationType::Add,
                _ => OperationType::Multiply,
            };
            let second = match operation_elems.next().unwrap() {
                "old" => OperationElem::Old,
                x => OperationElem::Number(x.parse().unwrap())
            };

            
            Some( Operation { operation: symbol, first: first, second: second } )
        },
        _ => None
    };
    Some((action, num, list, operation))

}

