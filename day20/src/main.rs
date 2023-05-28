use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let numbers = read_input()
        .expect("Should read file")
        .into_iter()
        .filter_map(|a| a.ok())
        .map(|a| a.parse::<i64>())
        .filter_map(|a| a.ok())
        .collect();

    let first = first_star(&numbers);
    println!("First star: {}", first);
    let second = second_star(&numbers);
    println!("Second star: {}", second);

}

fn first_star(numbers: &Vec<i64>) -> i64 {
    let indices: Vec<usize> = numbers
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect();
    let decoded = mix(&numbers, &indices);
    let old_zero_index = numbers
        .iter()
        .position(|&a| a == 0)
        .unwrap();

    let zero_index = decoded
        .iter()
        .position(|&a| a == old_zero_index)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|a| (a + zero_index) % decoded.len())
        .map(|a| decoded[a])
        .map(|a| numbers[a])
        .sum()
}

fn second_star(init_numbers: &Vec<i64>) -> i64 {
    const DECRYPTION_KEY: i64 = 811589153;
    let numbers: Vec<i64> = init_numbers
        .iter()
        .map(|a| a * DECRYPTION_KEY)
        .collect();

    let mut indices: Vec<usize> = numbers
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect();
    
    for _ in 0..10 {
        indices = mix(&numbers, &indices);
    }
    let old_zero_index = numbers
        .iter()
        .position(|&a| a == 0)
        .unwrap();

    let zero_index = indices
        .iter()
        .position(|&a| a == old_zero_index)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|a| (a + zero_index) % indices.len())
        .map(|a| indices[a])
        .map(|a| numbers[a])
        .sum()
}

fn mix(numbers: &Vec<i64>, indices: &Vec<usize>) -> Vec<usize> {
    let mut indices: Vec<usize> = indices.iter().map(|a| a.clone()).collect();
    for (i, n) in numbers.iter().enumerate() {
        let elem_index = indices
            .iter()
            .position(|a| a == &i)
            .unwrap();
        indices.remove(elem_index);
        let new_index = (elem_index as i64 + n).rem_euclid(indices.len() as i64) as usize;
        indices.insert(new_index, i);
    }
    indices
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
