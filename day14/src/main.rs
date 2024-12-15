use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    vector: (isize, isize),
    tile_row: isize,
    tile_col: isize,
}

impl Robot {
    fn new(input: &str, rows: isize, cols: isize) -> Self {
        let re = Regex::new(r"([p|v])=([-]?\d+),([-]?\d+)").unwrap();
        let values: Vec<(isize, isize)> = re
            .captures_iter(input)
            .map(|s| {
                (
                    s[3].to_string().parse::<isize>().unwrap().clone(),
                    s[2].to_string().parse::<isize>().unwrap().clone(),
                )

            })
            .collect();
        Robot {
            position: values[0],
            vector: values[1],
            tile_col: cols,
            tile_row: rows,
        }
    }

    fn move_n_times(&mut self, n: isize) {
        let row = match self.position.0 + self.vector.0 * n {
            x if x >= 0 => x % self.tile_row,
            x => self.tile_row - x % self.tile_row * -1,
        };
        let column = match self.position.1 + self.vector.1 * n {
           y if y >= 0 => y % self.tile_col,
           y => self.tile_col - y % self.tile_col * -1,
        };
        self.position = (row, column);
    }
}

fn read_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn look_up_quadrants(robots: &Vec<Robot>) -> isize {
    let middle_row = 4;
    let middle_col = 5;

    let quadrant_1 = robots.iter().filter(|x|  x.position.0 < middle_row && x.position.1 < middle_col).count();
    let quadrant_2 = robots.iter().filter(|x| x.position.0 > middle_row && x.position.1 < middle_col).count();
    let quadrant_3 = robots.iter().filter(|x| x.position.0 < middle_row && x.position.1 > middle_col).count();
    let quadrant_4 = robots.iter().filter(|x| x.position.0 > middle_row && x.position.1 > middle_col).count();
    println!("Quad: {},{},{},{}", quadrant_1, quadrant_2, quadrant_3, quadrant_4);
    return (quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4) as isize;
}

fn main() {
    let input = read_file("test.txt");
    let mut robots: Vec<Robot> = input.iter().map(|r| Robot::new(r, 7, 11)).collect();
    robots.iter_mut().for_each(|r| r.move_n_times(100));
    let score =  look_up_quadrants(&robots);
    println!("{}", score);
}
