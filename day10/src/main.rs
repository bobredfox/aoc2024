use file_reader::file_handling::give_lines_of_file;
use std::{collections::{HashMap, VecDeque}, usize};

mod file_reader;


type Position = (usize, usize);

#[derive(Debug)]
struct LavaMap {
    tiles: HashMap<Position, usize>
}

fn create_Map(input: Vec<String>) -> LavaMap {
    let mut data: HashMap<Position, usize> = HashMap::new();

    input.iter().enumerate()
    .for_each(|(x, s)| 
    s.chars()
    .enumerate()
    .map(|(y, c)| (y, c.to_digit(10).unwrap()))
    .for_each(|(y, h)| {data.insert((x,y), h.try_into().unwrap());}));
    return LavaMap {tiles:data};
}

fn find_trails(lava_map: &LavaMap) -> usize {
    let starting_points: Vec<Position> = lava_map.tiles.iter().filter(|(_, value)| value == &&0).map(|(p,_)| p.clone()).collect();
    println!("{:?}", starting_points);

    let mut scores = 0;

    for start in starting_points.iter() {
        scores += trail_walker(lava_map, start.clone())
    }

    return scores;
}

fn trail_walker(lava_map: &LavaMap, starting_point: Position) -> usize {
    let mut schlange: VecDeque<Position> = VecDeque::new();
    let mut visited: Vec<Position> = Vec::new();

    let mut counter = 0;

    visited.push(starting_point);
    schlange.push_back(starting_point);

    while let Some(point) = schlange.pop_front() {
        visited.push(point.clone());
        let height = lava_map.tiles.get(&point).unwrap();
        let neighbours = get_neighbours(&lava_map, point);

        for all in neighbours.iter() {
            if !visited.contains(all) {
                if let Some(point_b) = lava_map.tiles.get(all) {
                    if point_b == &9 && point_b - height == 1 {
                        counter += 1;
                        visited.push(all.clone());
                    }
                    if point_b > height && point_b - height == 1 && !visited.contains(all) {
                        println!("Pushed: {:?} with {}", all, point_b);
                        schlange.push_back(all.clone());
                    };
                }
            }
            
        }
    }

    return counter;

}

fn get_neighbours(lava_map: &LavaMap, point: Position) -> Vec<Position> {
    // x + 1, x - 1, y + 1, y - 1
    match point {
        (0, 0) => vec![(0,1), (1,0)],
        (0, y) => vec![(0, y+1), (0, y-1), (1, y)],
        (x, 0) => vec![(x + 1, 0), (x - 1, 0), (x, 1)],
        (x,y) => vec![(x, y+1), (x, y-1), (x+1, y), (x-1,y)],
    }
}


fn main() {
    let input = give_lines_of_file("test.txt").unwrap();
    let data = create_Map(input);
    let trails = find_trails(&data);
    println!("Hello, world!, {}", trails);

}
