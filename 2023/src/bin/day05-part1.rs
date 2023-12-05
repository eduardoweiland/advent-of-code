use std::{
    convert::Infallible,
    io::{self, Lines, StdinLock},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct MapEntry {
    dst_start: usize,
    src_start: usize,
    range_length: usize,
}

impl FromStr for MapEntry {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        Ok(Self {
            dst_start: iter.next().unwrap().parse().unwrap(),
            src_start: iter.next().unwrap().parse().unwrap(),
            range_length: iter.next().unwrap().parse().unwrap(),
        })
    }
}

impl MapEntry {
    fn try_map(&self, src_number: usize) -> Option<usize> {
        if src_number >= self.src_start && src_number < self.src_start + self.range_length {
            Some(self.dst_start + src_number - self.src_start)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn from_stdin(lines: &mut Lines<StdinLock<'static>>) -> Option<Self> {
        let mut map = Self::default();

        // skip header
        lines.next();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            } else {
                map.entries.push(line.parse().unwrap());
            }
        }

        if map.entries.is_empty() {
            None
        } else {
            Some(map)
        }
    }

    fn map_numbers(&self, numbers: Vec<usize>) -> Vec<usize> {
        numbers
            .iter()
            .map(|src_number| {
                self.entries
                    .iter()
                    .find_map(|entry| entry.try_map(*src_number))
                    .unwrap_or(*src_number)
            })
            .collect()
    }
}

fn main() {
    let mut lines = io::stdin().lines();

    let mut source_numbers: Vec<_> = lines
        .by_ref()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    // empty line
    lines.next();

    while let Some(map) = Map::from_stdin(&mut lines) {
        source_numbers = map.map_numbers(source_numbers);
    }

    let min = source_numbers.iter().reduce(std::cmp::min).unwrap();
    println!("{}", min);
}
