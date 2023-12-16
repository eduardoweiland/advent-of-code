use std::io::stdin;

type Coord = (isize, isize);

struct Maze(Vec<Vec<char>>);

impl Maze {
    fn find_start(&self) -> Coord {
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.0[y][x] == 'S' {
                    return (y as isize, x as isize);
                }
            }
        };

        unreachable!("input should always have start point");
    }

    fn get(&self, coord: Coord) -> Option<char> {
        match self.0.get(coord.0 as usize) {
            Some(line) => line.get(coord.1 as usize).copied(),
            None => None,
        }
    }

    fn find_connecteds(&self, current: Coord) -> (Coord, Coord) {
        let north = (current.0 - 1, current.1);
        let south = (current.0 + 1, current.1);
        let west = (current.0, current.1 - 1);
        let east = (current.0, current.1 + 1);

        match self.get(current) {
            Some('|') => (north, south),
            Some('-') => (east, west),
            Some('L') => (north, east),
            Some('J') => (north, west),
            Some('7') => (south, west),
            Some('F') => (south, east),
            Some('S') => {
                let connecteds = vec![north, south, west, east];
                let connecteds: Vec<_> = connecteds.iter().filter(|&c| self.get(*c).unwrap_or('.') != '.').collect();
                (**connecteds.first().unwrap(), **connecteds.last().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let maze = Maze(stdin().lines().map(|l| l.unwrap().chars().collect()).collect());
    let start = maze.find_start();
    let mut previous = start;
    let mut current = start;
    let mut iterations = 0;

    loop {
        let connecteds = maze.find_connecteds(current);
        let next = if connecteds.0 == previous { connecteds.1 } else { connecteds.0 };

        if next == start {
            break;
        } else {
            previous = current;
            current = next;
            iterations += 1;
        }

        // println!("iter #{}", iterations);
        // println!("prev {:?}", previous);
        // println!("curr {:?}", current);
        // println!("next {:?}", next);
    }

    let distance = (iterations as f32 / 2.0).ceil();
    println!("distance = {}", distance);
}
