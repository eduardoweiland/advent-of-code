use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(grid: &Vec<Vec<char>>) -> u32 {
    let mut found = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            Dir::all().iter().for_each(|dir| {
                let handle = Handle {
                    grid,
                    pos: (x as i32, y as i32),
                    dir: *dir,
                };

                let letters: Vec<_> = handle.into_iter().take(4).collect();
                if letters == &['X', 'M', 'A', 'S'] {
                    found += 1;
                }
            });
        }
    }

    found
}

#[aoc(day4, part2)]
pub fn solve_part2(grid: &Vec<Vec<char>>) -> u32 {
    let mut found = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] == 'A' {
                let ul = grid[y - 1][x - 1];
                let ur = grid[y - 1][x + 1];
                let dl = grid[y + 1][x - 1];
                let dr = grid[y + 1][x + 1];

                if (ul == 'M' && dr == 'S') || (ul == 'S' && dr == 'M') {
                    if (ur == 'M' && dl == 'S') || (ur == 'S' && dl == 'M') {
                        found += 1;
                    }
                }
            }
        }
    }

    found
}

#[derive(Clone, Copy)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Dir {
    pub fn all() -> Vec<Self> {
        vec![
            Dir::Right,
            Dir::Left,
            Dir::Up,
            Dir::Down,
            Dir::UpRight,
            Dir::UpLeft,
            Dir::DownRight,
            Dir::DownLeft,
        ]
    }
}

struct Handle<'a> {
    grid: &'a Vec<Vec<char>>,
    pos: (i32, i32),
    dir: Dir,
}

impl Handle<'_> {
    fn advance(&mut self) {
        let (x, y) = self.pos;
        self.pos = match self.dir {
            Dir::Right => (x + 1, y),
            Dir::Left => (x - 1, y),
            Dir::Down => (x, y + 1),
            Dir::Up => (x, y - 1),
            Dir::DownRight => (x + 1, y + 1),
            Dir::DownLeft => (x - 1, y + 1),
            Dir::UpRight => (x + 1, y - 1),
            Dir::UpLeft => (x - 1, y - 1),
        };
    }
}

impl Iterator for Handle<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.grid
            .get(self.pos.0 as usize)
            .and_then(|row| row.get(self.pos.1 as usize))
            .copied()
            .and_then(|res| {
                self.advance();
                Some(res)
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 18);
    }

    #[test]
    fn it_solves_part2() {
        let answer = solve_part2(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 9);
    }
}
