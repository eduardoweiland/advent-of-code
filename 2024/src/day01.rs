use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let mut iter = l
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip()
}

#[aoc(day1, part1)]
pub fn solve_part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    to_sorted(left)
        .iter()
        .zip(to_sorted(right).iter())
        .fold(0, |sum, (l, r)| sum + l.abs_diff(*r))
}

#[inline]
fn to_sorted(vec: &Vec<u32>) -> Vec<u32> {
    let mut clone = vec.clone();
    clone.sort();
    clone
}

#[aoc(day1, part2)]
pub fn solve_part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut counts_in_right_list: HashMap<u32, u32> = HashMap::new();

    for r in right {
        *counts_in_right_list.entry(*r).or_insert(0) += 1;
    }

    left.iter().fold(0, |sum, l| {
        sum + l * counts_in_right_list.get(l).unwrap_or(&0)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn it_parses_input() {
        let (left, right) = parse_input(EXAMPLE_INPUT);
        assert_eq!(left, [3, 4, 2, 1, 3, 3]);
        assert_eq!(right, [4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn it_solves_part1() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let answer = solve_part1(&(left, right));
        assert_eq!(answer, 11);
    }

    #[test]
    fn it_solves_part2() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let answer = solve_part2(&(left, right));
        assert_eq!(answer, 31);
    }
}
