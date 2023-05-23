use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut result = 0;
    let mut i = 1;
    while let (Some(Ok(line1)), Some(Ok(line2)), _) = (lines.next(), lines.next(), lines.next()) {
        if test_order(&line1, &line2) {
            result += i;
        }
        i += 1;
    }
    println!("Result: {}", result);
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

fn test_order(line1: &String, line2: &String) -> bool {
    let chars1 = line1.as_bytes();
    let chars2 = line2.as_bytes();
    let len = chars1.len();
    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < len{
       if chars1[i1] == chars2[i2] {
           i1+= 1;
           i2+= 1;
       } else if chars1[i1] == 44 { // skip comma
           i1+= 1;
       } else if chars2[i2] == 44 {
           i2+= 1;
       } else if chars1[i1] == 93 { // ascii for ]
           return true;
       } else if chars2[i2] == 93 {
           return false;
       } else if chars1[i1] == 91 { // ascii for [
           if chars1[i1+1] == 93 {
               return true;
           } else if chars1[i1+1] != chars2[i2] {
               i1+= 1;
           } else if chars2[i1+2] == 93 {
               i1+= 3;
           } else {
               return false;
           }
       } else if chars2[i2] == 91 { // ascii for [
           if chars2[i2+1] == 93 {
               return false;
           } else if chars2[i2+1] != chars1[i1] {
               i2+= 1;
           } else if chars2[i2+2] == 93 {
               i2+= 3;
           } else {
               return true;
           }
       } else {
           return chars1[i1] < chars2[i2];
       }
    }
    return false;
}
