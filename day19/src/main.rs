use std::fs;

type Towels = Vec<String>;
type TowelPattern = Vec<String>;

fn file_read(path: &str) -> (Towels, TowelPattern) {
    let file_content = fs::read_to_string(path).expect("File not found.");
    let split: Vec<&str> = file_content.split("\n\n").collect();
    return (parse_towels(split[0]), parse_pattern(split[1]));
}

fn parse_towels(input: &str) -> Towels {
    let mut towels: Towels = input.split(", ").map(|s| s.to_string()).collect();
    towels.sort_by(|a,b| b.len().cmp(&a.len()));
    return towels;
}

fn parse_pattern(input: &str) -> TowelPattern {
    input.lines().map(|s| s.to_string()).collect()
}

fn pattern_can_be_made(pattern: &str, towels: &Vec<String>) -> bool{
    
    let mut combined = String::from("");
    while towels.iter().any(|t| add_towel(pattern, t, &mut combined)){
    };
    return combined == pattern;
}

fn add_towel(pattern: &str, towel: &String, combined: &mut String) -> bool {
    let test = combined.clone() + towel;
    if test.len() > pattern.len() {
        return false;
    } else {
    let slice = &pattern[..test.len()];
    match test.as_str() {
        s if s == slice => {combined.push_str(&towel);
            return true },
        _ => false,
    }
}
}

fn main() {
    let (towels, patterns) = file_read("input.txt");
    let possible: Vec<bool> = patterns.iter().map(|p| pattern_can_be_made(p, &towels)).collect();
    let possible_solutions = possible.iter().filter(|x| **x).count();
    println!("Towels: {:?}", possible_solutions);
}
