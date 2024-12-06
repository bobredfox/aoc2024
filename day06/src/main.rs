use std::fs;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Clone)]
struct Board {
    rows: usize,
    columns: usize,
    obstacles: Vec<Position>,
    guard: Position,
    guard_direction: Direction,
    guard_visited_places: Vec<Position>,
    left_board: bool
}

impl Board {
    fn new(input: &str) -> Board {
        let lines: Vec<&str> = input.lines().collect();

        let obstacles: Vec<Position> = lines
            .iter()
            .enumerate()
            .flat_map(|(row, x)| {
                x.chars()
                    .enumerate()
                    .filter(|(_, y)| y == &'#')
                    .map(|(col, _)| Position(row, col))
                    .collect::<Vec<Position>>()
            })
            .collect();

        let guard: Position = lines
            .iter()
            .enumerate()
            .map(|(row, x)| 
                x.chars()
                    .enumerate()
                    .find(|(_, y)| y == &'^')
                    .map(|(col, _)| Position(row, col))
                    
            )
            .flatten().next().unwrap();

        Board {
            rows: lines.len() - 1,
            columns: lines[0].len() - 1,
            obstacles,
            guard,
            guard_direction: Direction::North,
            guard_visited_places: vec![guard],
            left_board: false
        }
    }

    fn evaluate_board(mut self) -> Self {
    let mut steps = 0; 
      while !self.left_board && steps <= 10000 {
        self.move_guard();
        steps = steps + 1;
      }
      return self;

    }

    fn move_guard(&mut self) {
    // Leaving the map?
    if self.guard.0 == 0 && self.guard_direction == Direction::North
    || self.guard.0 == self.rows && self.guard_direction == Direction::South
    || self.guard.1 == self.columns && self.guard_direction == Direction::East
    || self.guard.1 == 0 && self.guard_direction == Direction::West{
            self.left_board = true;       
    } else {
    let next_position = match self.guard_direction {
        Direction::North => Position(self.guard.0 - 1, self.guard.1),
        Direction::South => Position(self.guard.0 + 1, self.guard.1),
        Direction::East => Position(self.guard.0, self.guard.1 + 1),
        Direction::West => Position(self.guard.0, self.guard.1 - 1),
        Direction::None => self.guard
    };

    if self.obstacles.contains(&next_position) {
        self.guard_direction = match self.guard_direction{ 
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
        Direction::None => Direction::None 
        };
    } else {

    self.guard_visited_places.push(next_position.clone());
    self.guard = next_position;
    }
    }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd)]
struct Position(usize, usize);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("File not found!");
    let board = Board::new(&input);
    let mut result = board.clone().evaluate_board();

    result.guard_visited_places.sort();
    result.guard_visited_places.dedup();
    let count = result.guard_visited_places.len();
    println!("{} unique Places", count); 
    let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
    let mut children = Vec::new();

    for position in result.guard_visited_places.iter() {
        let tx = tx.clone();
        let mut new_board = board.clone();
        let new_pos = position.clone();
        new_board.obstacles.push(position.clone());
        let handle = thread::spawn(move || {
            let new_result = new_board.evaluate_board();
            tx.send(new_result.left_board).unwrap();
            println!("Thread finished!: {}, {:?}", new_result.left_board, new_pos);
        });
        children.push(handle);
    }
    let mut obstacle_results: Vec<bool> = Vec::new();
    let mut count = 0; 
    while count < result.guard_visited_places.len() {
        match rx.try_recv() {
            Ok(x) => {obstacle_results.push(x); count += 1;},
            Err(_) => (),
        }
    }

    let loops = obstacle_results.iter().filter(|x| !*x).count();

    println!("Visited_places: {:?}, found {} for loops", count, loops);
}
