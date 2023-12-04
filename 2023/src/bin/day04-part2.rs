use std::{convert::Infallible, io, str::FromStr};

#[derive(Debug)]
struct Card {
    instances: u32,
    winning_numbers: Vec<u8>,
    numbers_you_have: Vec<u8>,
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace().skip(2);

        Ok(Self {
            instances: 1,
            winning_numbers: iter.by_ref().map_while(|n| n.parse().ok()).collect(),
            numbers_you_have: iter.map(|n| n.parse().unwrap()).collect(),
        })
    }
}

impl Card {
    fn matches(&self) -> usize {
        self.numbers_you_have
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

fn main() {
    let mut cards: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Card>().unwrap())
        .collect();

    for i in 1..=cards.len() {
        let range = i..(i + cards[i - 1].matches());
        let current_instances = cards[i - 1].instances;
        cards[range]
            .iter_mut()
            .for_each(|copy| copy.instances += current_instances);
    }

    println!("{}", cards.iter().map(|c| c.instances).sum::<u32>());
}
