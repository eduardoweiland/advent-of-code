use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|l| l.chars().map(Position::from_char).collect())
        .collect()
}

#[aoc(day6, part1)]
fn solve_part1(map: &Vec<Vec<Position>>) -> u32 {
    let mut map = map.clone();
    let (mut guard_pos, mut guard_dir) = find_guard(&map);

    let mut visited = 1;
    map[guard_pos.1][guard_pos.0] = Position::Visited;

    while let Some(new_pos) = next_guard_pos(guard_pos, guard_dir, &map) {
        match map[new_pos.1][new_pos.0] {
            Position::NotVisited => {
                map[new_pos.1][new_pos.0] = Position::Visited;
                visited += 1;
                guard_pos = new_pos;
            }
            Position::Visited => guard_pos = new_pos,
            Position::Obstacle => guard_dir = guard_dir.turn(),
            Position::Guard(_) => unreachable!(),
        }
    }

    visited
}

#[aoc(day6, part2)]
fn solve_part2(map: &Vec<Vec<Position>>) -> u32 {
    let mut possible_loops = 0;

    // a simple brute-force
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match &map[y][x] {
                Position::NotVisited => {
                    let mut map = map.clone();
                    map[y][x] = Position::Obstacle;

                    println!("checking for loops with obstacle at {x}x{y}");
                    if contains_loop(&map) {
                        possible_loops += 1;
                    }
                }
                _ => {}
            }
        }
    }

    possible_loops
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn turn(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Position {
    NotVisited,
    Visited,
    Obstacle,
    Guard(Dir),
}

impl Position {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Position::NotVisited,
            '#' => Position::Obstacle,
            '^' => Position::Guard(Dir::Up),
            '>' => Position::Guard(Dir::Right),
            'v' => Position::Guard(Dir::Down),
            '<' => Position::Guard(Dir::Left),
            _ => unreachable!(),
        }
    }
}

fn find_guard(map: &Vec<Vec<Position>>) -> ((usize, usize), Dir) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Position::Guard(dir) = &map[y][x] {
                return ((x, y), *dir);
            }
        }
    }

    unreachable!();
}

fn next_guard_pos(
    (x, y): (usize, usize),
    dir: Dir,
    map: &Vec<Vec<Position>>,
) -> Option<(usize, usize)> {
    match dir {
        Dir::Up if y > 0 => Some((x, y - 1)),
        Dir::Right if x < map.len() - 1 => Some((x + 1, y)),
        Dir::Down if y < map[0].len() - 1 => Some((x, y + 1)),
        Dir::Left if x > 0 => Some((x - 1, y)),
        _ => None,
    }
}

fn contains_loop(map: &Vec<Vec<Position>>) -> bool {
    let (mut guard_pos, mut guard_dir) = find_guard(&map);
    let mut visited_positions = vec![];

    visited_positions.push((guard_pos, guard_dir));

    while let Some(new_pos) = next_guard_pos(guard_pos, guard_dir, &map) {
        match map[new_pos.1][new_pos.0] {
            Position::Obstacle => guard_dir = guard_dir.turn(),
            _ => {
                if visited_positions.contains(&(new_pos, guard_dir)) {
                    return true;
                } else {
                    visited_positions.push((new_pos, guard_dir));
                    guard_pos = new_pos;
                }

                if visited_positions.len() > map.len() * map[0].len() * 4 {
                    return false;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 41);
    }

    #[test]
    fn it_solves_part2() {
        let answer = solve_part2(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 6);
    }
}
