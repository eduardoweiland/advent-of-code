use std::{cmp::Ordering, collections::HashMap, convert::Infallible, io::stdin, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Card(char);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        static ORDER: &str = "AKQT98765432J";
        ORDER.find(self.0).cmp(&ORDER.find(other.0))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Type {
    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut map: HashMap<Card, u8> = HashMap::new();
        let mut jokers = 0;

        cards.iter().for_each(|card| {
            if card.0 == 'J' {
                jokers += 1;
            } else {
                map.entry(*card).and_modify(|c| *c += 1).or_insert(1);
            }
        });

        match map.len() {
            0 => Self::FiveOfAKind, // all jokers
            1 => Self::FiveOfAKind,
            2 => match map.values().any(|x| *x >= 4 - jokers) {
                true => Self::FourOfAKind,
                false => Self::FullHouse,
            },
            3 => match map.values().any(|x| *x >= 3 - jokers) {
                true => Self::ThreeOfAKind,
                false => Self::TwoPair,
            },
            4 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    hand_type: Type,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cards.cmp(&other.cards),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let cards = iter.next().unwrap().chars().map(|c| Card(c)).collect();
        let bid = iter.next().unwrap().parse().unwrap();
        let hand_type = Type::from_cards(&cards);

        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

fn main() {
    let mut hands: Vec<Hand> = stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    hands.sort();
    hands.reverse();

    // hands.iter().for_each(|hand| println!("{:?}", hand));

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |tot, (rank, hand)| tot + (rank as u64 + 1) * hand.bid);

    println!("total winnings = {}", winnings);
}
