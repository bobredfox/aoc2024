use std::fs;
use regex::Regex;

fn read_file(path: &str) -> Vec<String> {
    let output = fs::read_to_string(path).expect("This should work!");
    return output.lines().map(|x| String::from(x)).collect();
}

fn get_muls(input: String) -> Option<i32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let values: Vec<(&str, &str)> = re.captures_iter(&input).map(|m| { let(_, [x,y]) = m.extract(); (x,y)}).collect();
    
    if values.len() == 0 {
        return None;
    } else {
        println!("{:?}", values);
        let output: i32 = values.iter().map(|x| x.0.parse::<i32>().expect("Found {x}") * x.1.parse::<i32>().unwrap()).sum();
        return Some(output);
    }
}

fn main() {
    let file_input = read_file("input.txt");
    let muls: i32 = file_input.iter().map(|x| get_muls(x.to_string()).unwrap()).sum();
    println!("mul_sum: {}!", muls);
}
