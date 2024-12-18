use core::num;
use std::fs;

type Position = (usize, usize);
type Memory = Vec<Vec<bool>>;

fn read_input(path: &str) -> Vec<Position> {
    let input = fs::read_to_string(path).expect("File not found!");
    let output: Vec<&str> = input.lines().collect();
    let mut outputvec: Vec<(usize,usize)> = Vec::new();
    for &numbers in output.iter() {
       let parts: Vec<&str> = numbers.clone().split(",").collect();
        outputvec.push((parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap()));
    }
    return outputvec;
}

fn init_memory(size: usize) -> Memory {
    vec![vec![false; size]; size]
}


fn main() {
    let drops = read_input("test.txt");
    let mut memory = init_memory(7);
    println!("{:?}", memory);
}
