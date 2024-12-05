use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Rules {
    left: usize,
    right: usize,
}

struct Print_Qeue {
    data: Vec<usize>,
}

fn read_file(path: &str) -> (String, String) {
    let output = fs::read_to_string(path).expect("No file found!");
    let parts: Vec<&str> = output.split("\n\n").collect();
    return (String::from(parts[0]), String::from(parts[1]));
}

fn create_ruleset(rule_string: String) -> Vec<Rules> {
    let lines: Vec<Rules> = rule_string
        .lines()
        .map(|s| {
            s.split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .map(|x: Vec<usize>| Rules {
            left: x[0],
            right: x[1],
        })
        .collect();
    return lines;
}

fn create_printqeue(print_string: String) -> Vec<Print_Qeue> {
    let lines: Vec<Print_Qeue> = print_string
        .lines()
        .map(|s| {
            s.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .map(|x: Vec<usize>| Print_Qeue { data: x })
        .collect();
    return lines;
}

fn check_printqeue(ruleset: &Vec<Rules>, print_qeue: &Print_Qeue) -> bool {
    let ohoh: Vec<&Rules> = ruleset
        .iter()
        .filter(|x| print_qeue.data.contains(&x.left) && print_qeue.data.contains(&x.right))
        .collect();
    return ohoh.iter().all(|x| check_rule(x, print_qeue));
}

fn check_wrong_printqeue(ruleset: &Vec<Rules>, print_qeue: &Print_Qeue) -> Vec<usize> {
    let ohoh: Vec<Rules> = ruleset
        .iter()
        .filter(|x| print_qeue.data.contains(&x.left) && print_qeue.data.contains(&x.right))
        .map(|r| r.clone())
        .collect();
    return sort_print_queue(&ohoh, print_qeue);
}
fn check_rule(rule: &Rules, print_qeue: &Print_Qeue) -> bool {
    let test = print_qeue
        .data
        .iter()
        .skip_while(|x| x != &&rule.left)
        .fold(false, |acc, x| if x == &rule.right { true } else { acc });
    return test;
}

fn check_rule_vector(rule: &Rules, print_qeue: &Vec<usize>) -> bool {
    let test = print_qeue
        .iter()
        .skip_while(|x| x != &&rule.left)
        .fold(false, |acc, x| if x == &rule.right { true } else { acc });
    return test;
}

fn sort_print_queue(ruleset: &Vec<Rules>, print_qeue: &Print_Qeue) -> Vec<usize> {
    let mut que: Vec<usize> = print_qeue.data.clone();

    while !ruleset.iter().all(|x| check_rule_vector(x, &que)) {
        for rule in ruleset {
            if check_rule_vector(rule, &que) {
            } else {
                let left_pos = que.iter().position(|x| x == &rule.left).unwrap();
                let right_pos = que.iter().position(|x| x == &rule.right).unwrap();
                que.swap(right_pos, left_pos);
            }
        }
    }
    return que;
}

fn main() {
    let input = read_file("input.txt");
    let ruleset = create_ruleset(input.0);
    let print_qeue = create_printqeue(input.1);

    let valid_qeues: usize = print_qeue
        .iter()
        .filter(|pq| check_printqeue(&&ruleset, pq))
        .map(|x| x.data[x.data.len() / 2])
        .sum();

    let wrong: usize = print_qeue
        .iter()
        .filter(|pq| !check_printqeue(&&ruleset, pq))
        .map(|x| check_wrong_printqeue(&ruleset, x))
        .map(|x| x[x.len() / 2])
        .sum();
    println!("Sum Right: {}, Sum Wrong {}", valid_qeues, wrong);
}
