use std::{io::stdin, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(String, u8),
    Remove(String),
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(['=', '-']);
        let label = parts.next().unwrap().to_string();

        match parts.next() {
            Some(n) if !n.is_empty() => Ok(Self::Add(label, n.parse()?)),
            _ => Ok(Self::Remove(label)),
        }
    }
}

impl Operation {
    fn apply(&self, boxes: &mut Vec<Vec<Lens>>) {
        match self {
            Self::Add(label, focal_length) => {
                let lens_box = &mut boxes[calc_hash(label)];
                let mut replaced = false;

                for lens in &mut *lens_box {
                    if lens.label == *label {
                        lens.focal_length = *focal_length;
                        replaced = true;
                    }
                }

                if !replaced {
                    lens_box.push(Lens {
                        label: label.clone(),
                        focal_length: *focal_length,
                    });
                }
            }
            Self::Remove(label) => {
                boxes[calc_hash(label)].retain(|lens| lens.label != *label);
            }
        }
    }
}

fn calc_hash(input: &str) -> usize {
    let mut hash = 0;

    input.bytes().for_each(|b| {
        hash += b as usize;
        hash *= 17;
        hash %= 256;
    });

    hash
}

fn calc_focusing_power(boxes: &Vec<Vec<Lens>>) -> u64 {
    boxes
        .iter()
        .enumerate()
        .map(|(box_number, r#box)| {
            (box_number as u64 + 1)
                * r#box
                    .iter()
                    .enumerate()
                    .map(|(slot, lens)| (slot as u64 + 1) * lens.focal_length as u64)
                    .sum::<u64>()
        })
        .sum()
}

fn main() {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .for_each(|op| {
            // println!("\nafter {op}");
            op.parse::<Operation>().unwrap().apply(&mut boxes);
            // boxes.iter().enumerate().for_each(|(i, b)| if b.len() > 0 { println!("#{i} {:?}", b); });
        });

    println!("answer = {}", calc_focusing_power(&boxes));
}
