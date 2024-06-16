use pathfinding::prelude::astar;
use std::io::stdin;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl Heading {
    fn inverse(&self) -> Self {
        match self {
            Heading::Right => Heading::Left,
            Heading::Left => Heading::Right,
            Heading::Up => Heading::Down,
            Heading::Down => Heading::Up,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Pos(u32, u32);

impl Pos {
    fn advance(&self, heading: &Heading, limit: &Pos) -> Option<Self> {
        match (heading, self.0, self.1) {
            (Heading::Up,    y, x) if y > 0 => Some(Pos(y - 1, x)),
            (Heading::Down,  y, x) if y < limit.0 => Some(Pos(y + 1, x)),
            (Heading::Left,  y, x) if x > 0 => Some(Pos(y, x - 1)),
            (Heading::Right, y, x) if x < limit.1 => Some(Pos(y, x + 1)),
            _ => None, // out of bounds
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Handle {
    pos: Pos,
    dest: Pos,
    heading: Option<Heading>,
    straight_seq: usize,
}

impl Handle {
    fn distance(&self) -> u32 {
        self.pos.0.abs_diff(self.dest.0) + self.pos.1.abs_diff(self.dest.1)
    }

    fn successors(&self, weights: &Vec<Vec<u32>>) -> Vec<(Handle, u32)> {
        [Heading::Up, Heading::Down, Heading::Left, Heading::Right]
            .into_iter()
            .filter(|heading| !self.heading.as_ref().is_some_and(|h| h.inverse() == *heading)) // cannot move to inverse heading
            .filter(|heading| !self.heading.as_ref().is_some_and(|h| *h == *heading && self.straight_seq == 10)) // cannot move to same heading more than ten blocks
            .filter(|heading| !self.heading.as_ref().is_some_and(|h| *h != *heading && self.straight_seq < 4)) // cannot move to other heading before four blocks
            .filter_map(|heading| self.pos.advance(&heading, &self.dest).map(|pos| (heading, pos)))
            .map(|(heading, pos)| self.moved_to(heading, pos))
            .map(|handle| {
                let weight = weights[handle.pos.0 as usize][handle.pos.1 as usize];
                (handle, weight)
            })
            .collect()
    }

    fn moved_to(&self, heading: Heading, pos: Pos) -> Self {
        Self {
            pos,
            dest: self.dest.clone(),
            straight_seq: match &self.heading {
                Some(h) if *h == heading => self.straight_seq + 1,
                _ => 1,
            },
            heading: Some(heading),
        }
    }
}

fn main() {
    let weights: Vec<Vec<_>> = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let start = Handle {
        pos: Pos(0, 0),
        dest: Pos(weights.len() as u32 - 1, weights[0].len() as u32 - 1),
        heading: None,
        straight_seq: 0,
    };

    let res = astar(
        &start,
        |h| h.successors(&weights),
        |h| h.distance(),
        |h| h.pos == h.dest && h.straight_seq >= 4,
    )
    .expect("could not find path");

    println!("{}", res.1);
}
