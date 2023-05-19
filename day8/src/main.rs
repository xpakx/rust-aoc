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
    let mut max_scenic = 0;
    for x in 0..height {
        for y in 0..width {
            if x==0 || y==0 || x==height-1 || y==width-1 || test_field(x, y, &forest)
            {
                result += 1;
            }         
            let scenic_score = calculate_scenic_score(x, y, &forest);
            if scenic_score > max_scenic {
                max_scenic = scenic_score;
            }
        }
    }
    // print_forest(&forest);
    println!("Result: {}", result);
    println!("Scenic score: {}", max_scenic);
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

fn calculate_scenic_score(x: usize, y: usize, forest: &Vec<Vec<char>>) -> u32 {
    let number = forest[x][y].to_digit(10).unwrap();
    let left_trees = &forest[x][..y].iter().map(|c| {
        c.to_digit(10).unwrap()
    }).rev().collect();
    let left = calculate_partial_scenic_score(number, left_trees);
    let right_trees = &forest[x][y+1..].iter().map(|c| {
        c.to_digit(10).unwrap()
    }).collect();
    let right = calculate_partial_scenic_score(number, right_trees);
    let top_trees = &forest[..x].iter().map(|it| it[y]).map(|c| {
        c.to_digit(10).unwrap()
    }).rev().collect();
    let top = calculate_partial_scenic_score(number, top_trees);
    let bottom_trees = &forest[x+1..].iter().map(|it| it[y]).map(|c| {
        c.to_digit(10).unwrap()
    }).collect();
    let bottom = calculate_partial_scenic_score(number, bottom_trees);
    return left*right*top*bottom;

}

fn calculate_partial_scenic_score(current: u32, trees: &Vec<u32> ) -> u32 {
    let mut result = 0;
    for tree in trees {
        result += 1;
        if *tree >= current {
            break;
        }
    }
    return result;
}
