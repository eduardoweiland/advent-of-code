use std::{convert::Infallible, io::stdin, str::FromStr};

struct Sequence {
    numbers: Vec<i64>,
}

impl FromStr for Sequence {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: s
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect(),
        })
    }
}

impl Sequence {
    fn new(numbers: Vec<i64>) -> Self {
        Self { numbers }
    }

    fn next(&self) -> i64 {
        let mut diffs = vec![];
        let mut is_all_zeroes = true;

        for i in 1..self.numbers.len() {
            let diff = self.numbers[i] - self.numbers[i - 1];
            is_all_zeroes = is_all_zeroes && diff == 0;
            diffs.push(diff);
        }

        if is_all_zeroes {
            *self.numbers.last().unwrap()
        } else {
            Self::new(diffs).next() + self.numbers.last().unwrap()
        }
    }
}

fn main() {
    let sum: i64 = stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Sequence>().unwrap())
        .map(|s| s.next())
        .sum();
    println!("sum = {}", sum);
}
