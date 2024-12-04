use std::fs;

fn main() {
    let board= Board::new("input");
    let words = board.find_words();
    println!("Kandidaten: {:?}", words);
}

#[derive(Debug, Clone)]
struct Position(usize, usize);

struct Board {
    data: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
}


impl Board {
    fn new(path: &str) -> Board {
        let data = read_file(path);
        Board{
            data: data.clone(),
            rows: data.len() - 1,
            columns: data[0].len() - 1,
        }
    }

    fn find_candidates(&self, letter: char) -> Vec<Position> {
        self.data.iter().enumerate().map(|(x, val)| {
           val.iter().enumerate().filter(|(_, c)| **c == letter).map(|(y,_)| Position(x,y)).collect::<Vec<Position>>()
        }).flatten().collect()
    }

    fn find_words(&self) -> usize {
       let movements: Vec<(isize, isize)> = vec![(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)];
       let candidates = self.find_candidates('X');
       println!("Kandidaten: {:?}", candidates);
       let is_inside: Vec<bool> = candidates.iter().map(|x| movements.iter().map(move |y| move_along(self, x.clone(), y))).flatten()
       .filter(|x| x == &true)
       .collect();
       return is_inside.len();
    }
}

fn move_along(board: &Board, candidate: Position, movement: &(isize, isize)) -> bool {
    let mut word: Vec<char> = "MAS".chars().collect();
    let mut actual_position = candidate.clone();
    let mut found_char = true;
    let mut reached_end = false;
    while found_char && !reached_end {
        let x_pos = actual_position.0 as isize + movement.0;
        let y_pos = actual_position.1 as isize + movement.1;
        if x_pos >= 0 && x_pos <= board.rows as isize && y_pos >= 0 && y_pos <= board.columns as isize {
            actual_position = Position(x_pos as usize, y_pos as usize);
            if word.len() > 0 {
                found_char = word[0] == board.data[actual_position.0][actual_position.1];
                word.remove(0);
            } else {
                reached_end = true;
                println!("Kandidat: {:?}, Ergebnis: {:?}, word: {:?}, richtung: {:?}", candidate, reached_end, word, movement);
            }
        } else {
            found_char = false;
            if word.len() == 0 {
                reached_end = true
            }
        }
    };
    
    return reached_end;
}

fn read_file(path: &str) -> Vec<Vec<char>> {
    let output = fs::read_to_string(path).expect("File not found!");
    return output.lines().map(|s| s.to_string().chars().collect()).collect();
}


