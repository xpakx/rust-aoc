use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    first_star();
}

fn first_star() {
    let lines = read_input().expect("Should read from file");
    for line in lines {
        if let Ok(line) = line {
            let instruction = parse_line(&line);
            if instruction.is_none() {
                print!("\n");
                continue;
            }
            let instruction = instruction.unwrap(); 

            println!("{:?}", instruction);
        }
    }
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

#[derive(Debug)]
struct Operation {
    operation: OperationType,
    first: OperationElem,
    second: OperationElem
}

#[derive(Debug)]
enum OperationType {
    Add,
    Multiply
}

#[derive(Debug)]
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


