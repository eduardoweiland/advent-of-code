use std::io::stdin;

type Coord = (isize, isize);

struct Maze(Vec<Vec<char>>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    coord: Coord,
    pipe: char,
}

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
    let mut maze = Maze(stdin().lines().map(|l| l.unwrap().chars().collect()).collect());
    let start = Node { coord: maze.find_start(), pipe: 'S' };
    let mut steps: Vec<Node> = vec![start];
    let mut previous = start.coord;
    let mut current = start.coord;

    loop {
        let connecteds = maze.find_connecteds(current);
        let next = if connecteds.0 == previous { connecteds.1 } else { connecteds.0 };

        if next == steps[0].coord {
            break;
        } else {
            previous = current;
            current = next;
            steps.push(Node {
                coord: next,
                pipe: maze.get(next).unwrap(),
            });
        }
    }

    // Clear pipes that are not in the loop
    for y in 0..maze.0.len() {
        for x in 0..maze.0[y].len() {
            if steps.iter().find(|node| node.coord == (y as isize, x as isize)).is_none() {
                maze.0[y][x] = '.';
            }
        }
    }

    let mut nodes_inside = 0;

    // Find nodes that are inside the loop
    for y in 0..maze.0.len() {
        for x in 0..maze.0[y].len() {
            if maze.0[y][x] != '.' {
                continue;
            }

            let mut inside = false;

            for xx in 0..x {
                if maze.0[y][xx] == '|' || maze.0[y][xx] == 'L' || maze.0[y][xx] == 'J' || maze.0[y][xx] == 'S' {
                    inside = !inside;
                }
            }

            if inside {
                maze.0[y][x] = 'I';
                nodes_inside += 1;
            }
        }
    }

    // Print maze for visuals :)
    for y in 0..maze.0.len() {
        for x in 0..maze.0[y].len() {
            match maze.0[y][x] {
                '|' => print!("┃"),
                '-' => print!("━"),
                'L' => print!("┗"),
                'J' => print!("┛"),
                '7' => print!("┓"),
                'F' => print!("┏"),
                'S' => print!("╋"),
                n => print!("{}", n),
            }
        }
        println!();
    }

    println!("nodes inside = {}", nodes_inside);
}
