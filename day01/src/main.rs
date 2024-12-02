use std::fs;

struct ListOfNumbers {
    left_list: Vec<usize>,
    right_list: Vec<usize>,
}

fn read_file(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("No file found!");
    let lines: Vec<String> = content.lines().map(|x| String::from(x)).collect();
    return lines;
}

fn create_tuples(input: Vec<String>) -> ListOfNumbers {
    let mut list = ListOfNumbers {
        left_list: Vec::new(),
        right_list: Vec::new(),
    };

    for element in input.iter() {
        let numberVec = split_line(element);
        list.left_list.push(numberVec[0]);
        list.right_list.push(numberVec[1]);
    }
    return list;
}

fn split_line(input: &String) -> Vec<usize> {
    let values: Vec<String> = input.split_whitespace().map(|x| String::from(x)).collect();
    let number_values: Vec<usize> = values.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    return number_values;
}

fn evaluate_diff(mut input: ListOfNumbers) -> usize {
    input.left_list.sort();
    input.right_list.sort();

    let new_list: usize = input
        .left_list
        .iter()
        .zip(input.right_list.iter())
        .map(|x| calculator(x.0, x.1))
        .sum();

    return new_list;
}

fn calculator(x: &usize, y: &usize) -> usize {
    if x >= y {
        return x - y;
    } else {
        return y - x;
    }
}

fn similarity_score(input: ListOfNumbers) -> usize {
    let output = input
        .left_list
        .iter()
        .map(|x| count_number(x, &input.right_list) * x)
        .sum();
    return output;
}

fn count_number(number: &usize, list: &Vec<usize>) -> usize {
    return list
        .iter()
        .filter(|x| &number == x)
        .collect::<Vec<&usize>>()
        .len();
}

fn main() {
    let output = read_file("input.txt");
    let numbers = evaluate_diff(create_tuples(output.clone()));
    let similarity = similarity_score(create_tuples(output));
    println!("Distance: {}, Similarity: {}", numbers, similarity);
}
