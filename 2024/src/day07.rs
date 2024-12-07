use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Expression> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn solve_part1(expressions: &Vec<Expression>) -> u64 {
    expressions
        .iter()
        .filter(|expr| expr.is_solvable())
        .map(|expr| expr.result)
        .sum()
}

struct Expression {
    result: u64,
    operands: Vec<u64>,
}

impl FromStr for Expression {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut numbers = line.split([':', ' ']);

        let result = numbers.next().unwrap().parse().unwrap();
        let operands = numbers.skip(1).map(|num| num.parse().unwrap()).collect();

        Ok(Expression { result, operands })
    }
}

impl Expression {
    fn is_solvable(&self) -> bool {
        check(self.result, 0, &self.operands)
    }
}

fn check(expected: u64, accumulated: u64, remaining_operands: &[u64]) -> bool {
    if accumulated > expected {
        false
    } else if remaining_operands.len() > 0 {
        let with_add = accumulated + remaining_operands[0];
        let with_mult = accumulated * remaining_operands[0];
        check(expected, with_add, &remaining_operands[1..])
            || check(expected, with_mult, &remaining_operands[1..])
    } else {
        remaining_operands.len() == 0 && expected == accumulated
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 3749);
    }
}
