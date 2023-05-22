use std::fs;
use std::path::Path;
use std::io;
use std::collections::HashMap;

fn main() {
    let input = read_input().expect("Should read input from file");
    let input: Vec<&str> = input.trim().split('\n').collect();
    let input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    println!("{:?}", input);
    let rows = input.len();
    let columns = input[0].len();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut q: Vec<(usize, usize)> = Vec::new();
    for x in 0..rows {
        for y in 0..columns {
            if input[x][y] == 'S' {
                dist.insert((x,y), 0);
            } else {
                dist.insert((x,y), usize::MAX);
            }
            q.push((x,y));
        }
    }

    while !q.is_empty() {
        let x = q.iter().min_by_key(|a| dist.get(a).unwrap_or(&usize::MAX)).unwrap();
        let point = x.clone();
        println!("{:?}", x);
        let letter = input[point.0][point.1];
        if letter == 'E' {
            println!("{}", dist.get(x).unwrap());
            break;
        }
        q = q.iter().filter(|&a| a != x).map(|a| a.clone()).collect();
        let mut neighbours = Vec::new();
        if point.0 > 0 {
            let n = (point.0-1, point.1);
            if q.contains(&n) {
                neighbours.push(n);
            }
        }
        let alt = dist.get(&point).unwrap() + 1;
        println!("{}", alt);
        if point.0 < columns-1 {
            let n = (point.0+1, point.1);
            if q.contains(&n) {
                neighbours.push(n);
            }
        }
        if point.1 > 0 {
            let n = (point.0, point.1-1);
            if q.contains(&n) {
                neighbours.push(n);
            }
        }
        if point.1 < columns-1 {
            let n = (point.0, point.1+1);
            if q.contains(&n) {
                neighbours.push(n);
            }
        }

        for neighbour in neighbours {
            let n_dist = dist.get(&neighbour).unwrap();
            if alt < *n_dist {
                dist.insert(neighbour, alt);
                prev.insert(neighbour, point);
            }

        }



    }
}


fn read_input() -> Result<String, io::Error> {
    let file_path = Path::new("./input.txt");
    let file =  match fs::read_to_string(&file_path){
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };
    return Ok(file);
}
