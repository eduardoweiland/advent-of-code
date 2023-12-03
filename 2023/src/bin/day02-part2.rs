use std::{io, str::FromStr, num::ParseIntError, cmp};

fn main() {
    let mut sum: u64 = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let colon_index = line.find(':').unwrap();
        let mut max = GrabbedCubes::default();

        for hand in line[colon_index + 1..].split(';') {
            max = max.max(&hand.parse().unwrap())
        }

        sum += max.power();
    }

    println!("{}", sum);
}

#[derive(Debug, Default)]
struct GrabbedCubes {
    red: u64,
    green: u64,
    blue: u64,
}

impl FromStr for GrabbedCubes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Self::default();
        let mut iter = s.trim().split(&[' ', ',']).filter(|s| !s.is_empty());

        while let (Some(count), Some(color)) = (iter.next(), iter.next()) {
            match color {
                "red" => cubes.red = count.parse()?,
                "green" => cubes.green = count.parse()?,
                "blue" => cubes.blue = count.parse()?,
                _ => unreachable!("not a color {}", color),
            }
        }

        Ok(cubes)
    }
}

impl GrabbedCubes {
    fn max(&self, other: &Self) -> Self {
        Self {
            red: cmp::max(self.red, other.red),
            green: cmp::max(self.green, other.green),
            blue: cmp::max(self.blue, other.blue),
        }
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}
