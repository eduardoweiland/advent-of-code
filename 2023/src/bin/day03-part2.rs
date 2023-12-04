use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    io,
};

type Key = (usize, usize);

#[derive(Default)]
struct MaybeGear {
    pub part_numbers: Vec<u64>,
}

impl MaybeGear {
    fn add_part(&mut self, part_number: u64) {
        self.part_numbers.push(part_number);
    }

    fn is_gear(&self) -> bool {
        self.part_numbers.len() == 2
    }

    fn gear_ratio(&self) -> u64 {
        self.part_numbers.iter().product()
    }
}

fn main() {
    let schematic: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let mut maybe_gears: HashMap<Key, MaybeGear> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();

    for (row_idx, line) in schematic.iter().enumerate() {
        for a_match in re.find_iter(&line) {
            let col_range = (max(a_match.start(), 1) - 1)..min(a_match.end() + 1, line.len() - 1);
            let row_range = (max(row_idx, 1) - 1)..=min(row_idx + 1, schematic.len() - 1);
            let part_number = a_match.as_str().parse::<u64>().unwrap();

            for y in row_range {
                schematic[y][col_range.clone()]
                    .match_indices('*')
                    .for_each(|(x, _)| {
                        maybe_gears
                            .entry((x + col_range.start, y))
                            .or_default()
                            .add_part(part_number);
                    });
            }
        }
    }

    let sum: u64 = maybe_gears
        .iter()
        .filter_map(|(_, maybe_gear)| {
            if maybe_gear.is_gear() {
                Some(maybe_gear.gear_ratio())
            } else {
                None
            }
        })
        .sum();

    println!("{}", sum);
}
