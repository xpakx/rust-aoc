use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    first_star();
}

fn first_star() {
    let lines = read_input().expect("Should read from file");
    let mut clock_counter = 0;
    let mut reg_x = 1;
    let mut result = 0;
    for line in lines {
        if let Ok(line) = line {
            let instruction = parse_line(&line)
                .expect("Should parse instruction");
            let cycles_used = match instruction.0 {
                Action::Noop => 1,
                Action::AddX => 2
            };
            clock_counter += cycles_used;
            result += match test_cycles(clock_counter, cycles_used) {
                CycleType::Normal => 0,
                CycleType::Special => i64::from(reg_x)*i64::from(clock_counter),
                CycleType::SpecialJumpedOver => i64::from(reg_x)*i64::from(clock_counter-1),
            };
            reg_x += instruction.1;

        }
    }
    println!("Result: {}", result);
}

enum CycleType {
    Normal,
    Special,
    SpecialJumpedOver,
}

fn test_cycles(new: u32, cycles_used: u32) -> CycleType {
    let new = i64::from(new);
    let old_is = (cycles_used != 1) && (new-21) % 40 == 0;
    let new_is = (new-20) % 40 == 0;
    match (old_is, new_is) {
        (_, true) => CycleType::Special,
        (true, _) => CycleType::SpecialJumpedOver,
        (_, _) => CycleType::Normal
    }
}

enum Action {
    Noop,
    AddX,
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

fn parse_line(line: &String) -> Option<(Action, i32)> {
    let mut parsed = line.split(" ");
    let action = match parsed.next() {
        Some(p) => match p {
            "noop" => Action::Noop,
            "addx" => Action::AddX,
            _ => return None
        },
        None => return None
    };
    let num = match parsed.next() {
        Some(n) => match n.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0
        },
        None => 0
    };
    return Some((action, num));
}

