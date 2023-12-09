use std::{cmp, convert::Infallible, io::stdin, ops::Range, str::FromStr};

#[derive(Debug)]
struct MapEntry {
    from: Range<i64>,
    diff: i64,
}

impl FromStr for MapEntry {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace().map(|n| n.parse::<i64>().unwrap());

        let dst_start = iter.next().unwrap();
        let src_start = iter.next().unwrap();
        let len = iter.next().unwrap();

        Ok(Self {
            from: src_start..(src_start + len),
            diff: dst_start - src_start,
        })
    }
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn from_stdin(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Option<Self> {
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
            map.entries.sort_by(|a, b| a.from.start.cmp(&b.from.start));
            Some(map)
        }
    }

    fn map_range(&self, src: Range<i64>) -> Vec<Range<i64>> {
        let mut result = vec![];
        let mut src = src;

        for map in &self.entries {
            if src.start < map.from.start {
                let split = cmp::min(src.end, map.from.start);
                result.push(src.start..split);
                src = split..src.end;
            }

            if map.from.contains(&src.start) {
                let end = cmp::min(src.end, map.from.end);
                result.push((src.start + map.diff)..(end + map.diff));
                src = end..src.end;
            }

            if src.is_empty() {
                break;
            }
        }

        if !src.is_empty() {
            result.push(src);
        }

        result
    }
}

fn read_seeds() -> Vec<Range<i64>> {
    let mut numbers = String::new();
    stdin().read_line(&mut numbers).unwrap();

    let numbers = numbers
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut numbers = numbers.iter();
    let mut seeds = vec![];

    while let (Some(start), Some(length)) = (numbers.next(), numbers.next()) {
        let end = *start + *length;
        seeds.push(*start..end);
    }

    // println!("the seeds are: {:?}", seeds);

    seeds
}

fn read_maps() -> Vec<Map> {
    let mut lines = stdin().lines().skip(1);
    let mut maps = vec![];

    // println!("the maps are:");

    while let Some(map) = Map::from_stdin(&mut lines) {
        // println!("#{} {:?}", maps.len(), map);
        maps.push(map);
    }

    // println!();

    maps
}

fn main() {
    let mut seeds = read_seeds();
    let maps = read_maps();

    for (_index, map) in maps.iter().enumerate() {
        // println!("mapping iteration #{}", _index);
        // println!("input {:?}", seeds);

        seeds = seeds
            .iter()
            .flat_map(|range| map.map_range(range.clone()))
            .collect();

        // println!("results {:?}\n", seeds);
    }

    println!(
        "lowest location = {}",
        seeds.iter().map(|r| r.start).min().unwrap()
    );
}
