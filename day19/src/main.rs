use std::fs;
use itertools::Itertools;

type Towels = Vec<String>;
type TowelPattern = Vec<String>;


fn file_read(path: &str) -> (Towels, TowelPattern) {
    let file_content = fs::read_to_string(path).expect("File not found.");
    let split: Vec<&str> = file_content.split("\n\n").collect();
    return (parse_towels(split[0]), parse_pattern(split[1]));
}

fn parse_towels(input: &str) -> Towels {
    let mut towels: Towels = input.split(", ").map(|s| s.to_string()).collect();
    towels.sort_by(|a, b| b.len().cmp(&a.len()));
    return towels;
}

fn parse_pattern(input: &str) -> TowelPattern {
    input.lines().map(|s| s.to_string()).collect()
}


fn can_pattern_be_made(pattern: &str, towels: &Vec<String>) -> bool {
    let mut words: Vec<String> = vec![String::from("")];

    while words.iter().any(|w| w.len() < pattern.len()) && !words.iter().any(|w| w.as_str() == pattern) {
        words = compare_each_word(pattern, towels, words);
    }

    return words.iter().any(|w| w.as_str() == pattern);
}


fn can_pattern_be_made_how(pattern: &str, towels: &Vec<String>) -> usize {
    let mut words: Vec<String> = vec![String::from("")];
    let mut count = 0;
    while words.iter().any(|w| w.len() < pattern.len()) && words.len() > 0 {
        let temp = compare_each_word_double(pattern, towels, words);
        println!("Teste...");
        words = temp.iter().filter(|w| w.as_str() != pattern).map(|s| s.to_owned()).collect();
        count += temp.len() - words.len();
    }

    return count;
}


fn compare_each_word(pattern: &str, towels: &Vec<String>, words: Vec<String>) -> Vec<String> {
    words.iter().flat_map(|w| does_word(pattern, towels, w.as_str())).unique().collect()
}

fn compare_each_word_double(pattern: &str, towels: &Vec<String>, words: Vec<String>) -> Vec<String> {
    words.iter().flat_map(|w| does_word(pattern, towels, w.as_str())).collect()
}

fn does_word(pattern: &str, towels: &Vec<String>, word: &str) -> Vec<String> {
    towels.iter()
    .filter(|s| does_it_fit(pattern, s.as_str(), word))
    .map(|t| word.to_owned() + t).collect()
}


fn does_it_fit(pattern: &str, towel: &str, word: &str) -> bool {
    let new_word = word.to_owned() + towel;
    if new_word.len() <= pattern.len() {
        return pattern[..new_word.len()] == new_word;
    } else {
        return false;
    }
}


fn main() {
    let (towels, patterns) = file_read("input.txt");
    // let test: Vec<bool> = patterns.iter().map(|p| can_pattern_be_made(p.as_str(), &towels)).collect();
    let test_count: Vec<usize> = patterns.iter().enumerate().inspect(|(index, s)| println!("Processing {} of {}", index, patterns.len())).map(|(_, p)| can_pattern_be_made_how(p.as_str(), &towels)).collect();
    let numbers: usize = test_count.iter().sum();
    println!("Patterns: {}", numbers);
}
