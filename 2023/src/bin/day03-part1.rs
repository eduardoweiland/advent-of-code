use regex::Regex;
use std::{
    cmp::{max, min},
    io,
};

fn main() {
    let schematic: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let re = Regex::new(r"\d+").unwrap();
    let mut sum: u64 = 0;

    for (row_idx, line) in schematic.iter().enumerate() {
        for a_match in re.find_iter(&line) {
            let col_range = (max(a_match.start(), 1) - 1)..min(a_match.end() + 1, line.len() - 1);
            let row_range = (max(row_idx, 1) - 1)..=min(row_idx + 1, schematic.len() - 1);

            let should_sum = schematic[row_range].iter().any(|l| {
                l[col_range.clone()]
                    .find(|c: char| !c.is_ascii_digit() && c != '.')
                    .is_some()
            });

            if should_sum {
                sum += a_match.as_str().parse::<u64>().unwrap();
            }
        }
    }

    println!("{}", sum);
}
