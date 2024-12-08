use std::thread;
use std::sync::mpsc;
use crate::file_reader::file_handling::give_lines_of_file;
mod file_reader;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Equation{
    sum: usize,
    faktors: Vec<usize>
    }

impl Equation {
    fn new(line: &str) -> Self {
        let mut split = line.split(':');
        Equation { 
            sum: split.next().unwrap().parse::<usize>().expect("No Number!"),
            faktors: split.next().unwrap().split_whitespace().map(|s| s.parse::<usize>().expect("Error")).collect::<Vec<usize>>()    
        }
    }

    fn solve(&self, operators: Vec<&Funktion>) -> usize {
        let result = self.faktors.iter().skip(1).zip(operators).fold(self.faktors[0], |acc, (x, op)| op(acc, *x));
        return result; 

    }

    fn is_op(&self) -> bool {
        let add = Box::new(add);
        let mut op = vec![&add; self.faktors.len() - 1];
        // check if possible with add
        // check if possible with one mult
        // if bigger than sum stop
        let temp_result = self.solve(op);
        if temp_result == self.sum {
            return true;
        } else if temp_result > self.sum {
                return false;
            } else {
                return false; }
        }
    
}

type Funktion = Box<dyn Fn(usize, usize) -> usize>;

fn swap_ops(mut ops: Vec<Funktion>, index: usize) {
    let haha = std::mem::replace(&mut ops[index], Box::new(mult));
}

fn add(x: usize, y: usize) -> usize {
    x + y
}

fn mult(x: usize, y: usize) -> usize {
    x * y
}

fn main() {
    let input = match give_lines_of_file("test.txt") {
        Some(x) => x,
        None => Vec::new(),
    };
    let equation_vektor: Vec<Equation> = input.iter().map(|s| Equation::new(s)).collect();
    let true_equations: Vec<bool> = equation_vektor.iter().map(|e| e.is_op()).collect();
    println!("Equation: {:?}", equation_vektor);

}
