use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    first_star();
}

fn first_star() {
    let file_path = Path::new("./input.txt");
    let file =  match File::open(&file_path){
        Ok(file) => file,
        Err(error) => {
            eprintln!("Cannot load file: {}", error);
            return;
        }
    };
    let lines = io::BufReader::new(file).lines();
    let mut forest: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            forest.push(get_new_row(&line));
        }
    }
    let height: usize = forest.len();
    let width: usize = forest[0].len();
    let mut result = 0;
    for x in 0..height {
        for y in 0..width {
            if x==0 || y==0 || x==height-1 || y==width-1 || test_field(x, y, &forest)
            {
                result += 1;
            }         
        }
    }
    // print_forest(&forest);
    println!("Result: {}", result);
}

fn get_new_row(line: &String) -> Vec<char> {
    return line.chars().collect();
}

fn print_forest(forest: &Vec<Vec<char>>) {
    forest.into_iter().for_each(|it| {
        println!("{:?}", it);
    });
}

fn test_field(x: usize, y: usize, forest: &Vec<Vec<char>>) -> bool {
    let number = forest[x][y].to_digit(10).unwrap();
    let mut empty = true;
    for number2 in &forest[x][..y] {
        let number2 = number2.to_digit(10).unwrap();
        if number2 >= number {
            empty = false;
            break;
        }
    };
    if empty {
        return true;
    }
    empty = true;

    for number2 in &forest[x][y+1..] {
        let number2 = number2.to_digit(10).unwrap();
        if number2 >= number {
            empty = false;
            break;
        }
    };
    if empty {
        return true;
    }
    empty = true;

    let column: &Vec<char> = &forest[..x].iter().map(|it| it[y]).collect(); 
    for number2 in column {
        let number2 = number2.to_digit(10).unwrap();
        if number2 >= number {
            empty = false;
            break;
        }
    };
    if empty {
        return true;
    }
    empty = true;

    let column: &Vec<char> = &forest[x+1..].iter().map(|it| it[y]).collect(); 
    for number2 in column {
        let number2 = number2.to_digit(10).unwrap();
        if number2 >= number {
            empty = false;
            break;
        }
    };
    if empty {
        return true;
    }
 
    return false;
}
