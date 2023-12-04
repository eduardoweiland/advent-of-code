use std::{convert::Infallible, io, str::FromStr};

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u8>,
    numbers_you_have: Vec<u8>,
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace().skip(2);

        Ok(Self {
            winning_numbers: iter.by_ref().map_while(|n| n.parse().ok()).collect(),
            numbers_you_have: iter.map(|n| n.parse().unwrap()).collect(),
        })
    }
}

impl Card {
    fn points(&self) -> u64 {
        let matches = self
            .numbers_you_have
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
        if matches > 0 {
            u64::pow(2, matches as u32 - 1)
        } else {
            0
        }
    }
}

fn main() {
    let cards: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Card>().unwrap())
        .collect();
    println!("{}", cards.iter().map(|c| c.points()).sum::<u64>());
}
