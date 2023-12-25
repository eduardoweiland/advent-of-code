use std::io::stdin;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Heading {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Cell {
    content: char,
    visited_from: Vec<Heading>,
}

impl Cell {
    fn new(content: char) -> Self {
        Self {
            content,
            visited_from: vec![],
        }
    }

    fn visit(&mut self, from: Heading) -> Vec<Heading> {
        match self.visited_from.contains(&from) {
            true => vec![],
            false => {
                self.visited_from.push(from);

                match self.content {
                    '.' => vec![from],
                    '\\' => match from {
                        Heading::Right => vec![Heading::Down],
                        Heading::Left => vec![Heading::Up],
                        Heading::Up => vec![Heading::Left],
                        Heading::Down => vec![Heading::Right],
                    },
                    '/' => match from {
                        Heading::Right => vec![Heading::Up],
                        Heading::Left => vec![Heading::Down],
                        Heading::Up => vec![Heading::Right],
                        Heading::Down => vec![Heading::Left],
                    },
                    '|' => match from {
                        Heading::Up | Heading::Down => vec![from],
                        Heading::Right | Heading::Left => vec![Heading::Up, Heading::Down],
                    },
                    '-' => match from {
                        Heading::Right | Heading::Left => vec![from],
                        Heading::Up | Heading::Down => vec![Heading::Left, Heading::Right],
                    },
                    _ => vec![],
                }
            },
        }
    }

    fn is_energized(&self) -> bool {
        !self.visited_from.is_empty()
    }
}

#[derive(Clone, Debug)]
struct Coord(isize, isize);

impl Coord {
    fn next(&self, heading: &Heading) -> Self {
        match heading {
            Heading::Right => Self(self.0, self.1 + 1),
            Heading::Left => Self(self.0, self.1 - 1),
            Heading::Up => Self(self.0 - 1, self.1),
            Heading::Down => Self(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(cells: Vec<Vec<char>>) -> Self {
        Self {
            cells: cells.into_iter().map(|row| {
                row.into_iter().map(Cell::new).collect()
            }).collect(),
        }
    }

    fn at(&mut self, coord: &Coord) -> Option<&mut Cell> {
        match self.cells.get_mut(coord.0 as usize) {
            Some(row) => row.get_mut(coord.1 as usize),
            None => None,
        }
    }

    fn count_energized_cells(&self) -> u64 {
        self.cells.iter().flat_map(|row| row.iter().map(|cell| if cell.is_energized() { 1 } else { 0 })).sum()
    }
}

#[derive(Debug)]
struct Beam(Coord, Heading);

impl Beam {
    fn advance(&self, grid: &mut Grid) -> Vec<Beam> {
        let next_coord = self.0.next(&self.1);
        match grid.at(&next_coord) {
            None => vec![],
            Some(cell) => {
                // println!("found {cell:?} at {next_coord:?}");
                cell.visit(self.1).iter().map(|&heading| Beam(next_coord.clone(), heading)).collect()
            }
        }
    }
}

fn main() {
    let input: Vec<Vec<char>> = stdin().lines().map(|l| l.unwrap().chars().collect()).collect();
    let mut grid = Grid::new(input);
    let mut beams = vec![Beam(Coord(0, -1), Heading::Right)];

    // println!("{:?}", beams);
    while !beams.is_empty() {
        beams = beams.iter().flat_map(|beam| beam.advance(&mut grid)).collect();
        // println!("{:?}", beams);
    }

    // println!("{grid:?}");
    println!("answer = {}", grid.count_energized_cells());
}
