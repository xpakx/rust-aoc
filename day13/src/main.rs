use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::cmp::Ordering;

fn main() {
    let mut lines = read_input().expect("Should read file");
    let mut result = 0;
    let mut i = 1;
    let mut to_sort = Vec::new();
    while let (Some(Ok(line1)), Some(Ok(line2)), _) = (lines.next(), lines.next(), lines.next()) {
        let line1 = line1.replace("10", "A");
        let line2 = line2.replace("10", "A");
        if test_order(&line1, &line2) {
            result += i;
        }
        i += 1;
        to_sort.push(line1);
        to_sort.push(line2);
    }
    println!("Result 1: {}", result);
    to_sort.push(String::from("[[2]]"));
    to_sort.push(String::from("[[6]]"));
    to_sort.sort_by(|a, b| cmp_packets(a,b));
    let index1 = to_sort.iter().position(|p| p == "[[2]]");
    let index2 = to_sort.iter().position(|p| p == "[[6]]");
    if let (Some(index1), Some(index2)) = (index1, index2) {
        let key = (index1+1)*(index2+1);
        println!("Key: {}", key);
    }
}

fn cmp_packets(line1: &String, line2: &String) -> Ordering {
    if test_order(line1, line2) {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
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

fn test_order(line1: &String, line2: &String) -> bool {
    let chars1 = line1.as_bytes();
    let chars2 = line2.as_bytes();
    let len = chars1.len();
    let mut i1 = 0;
    let mut i2 = 0;
    let mut bracket1 = 0;
    let mut skip_brackets1 = false;
    let mut bracket2 = 0;
    let mut skip_brackets2 = false;
    while i1 < len {
        if skip_brackets1 {
            if chars1[i1] == 93 {
                bracket1 -= 1;
                i1 += 1;
            } else {
                return false;
            }
            if bracket1 == 0 {
                skip_brackets1 = false;
            }
        } else if skip_brackets2 {
            if chars2[i2] == 93 {
                bracket2 -= 1;
                i2 += 1;
            } else {
                return true;
            }
            if bracket2 == 0 {
                skip_brackets2 = false;
            }
        } else if bracket1 > 0 {
            if chars1[i1] == 44 {
                panic!("Shouldn't happen!");
            } else if chars1[i1] == 93 {
                return true;
            } else if chars1[i1] == 91 {
                bracket1 += 1;
                i1 += 1;
            } else {
                if chars1[i1] < chars2[i2] {
                    return true;
                } else if chars1[i1] > chars2[i2] {
                    return false;
                }
                i1 += 1;
                i2 += 1;
                skip_brackets1 = true
            }
        } else if bracket2 > 0 {
            if chars2[i2] == 44 {
                panic!("Shouldn't happen!");
            } else if chars2[i2] == 93 {
                return false;
            } else if chars2[i2] == 91 {
                bracket2 += 1;
                i2 += 1;
            } else {
                if chars1[i1] < chars2[i2] {
                    return true;
                } else if chars1[i1] > chars2[i2] {
                    return false;
                }
                i2 += 1;
                i1 += 1;
                skip_brackets2 = true
            }
        } else if chars1[i1] == chars2[i2] {
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
        } else if chars1[i1] == 91 {
            i1 += 1;
            bracket1 += 1;
        } else if chars2[i2] == 91 {
            i2 += 1;
            bracket2 += 1;
        } else {
            return chars1[i1] < chars2[i2];
        }
    }
    return false;
}
