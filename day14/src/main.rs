use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let mut lines = read_input().expect("Should read file");
    while let Some(Ok(line)) = lines.next() {
        let points = parse_line(&line);
        println!("{:?}", points);
    }
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

fn parse_line(line: &String) -> Vec<(usize, usize)> {
    let points = line.split("->")
        .map(|a| a.trim())
        .map(|a| a.split(",").collect())
        .map(|a: Vec<&str>| (a[0], a[1]))
        .map(|(a,b)| (a.parse::<usize>(), b.parse::<usize>()))
        .filter_map(|(a,b)| match (a,b) {
           (Ok(a), Ok(b)) => Some((a,b)),
           (_, _) => None
        })
        .collect();
   return points; 
}
