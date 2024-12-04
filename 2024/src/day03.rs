use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res},
    multi::{fold_many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Expr {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Expr> {
    map_res(
        delimited(
            tag("mul("),
            separated_pair(
                map_res(digit1, str::parse),
                tag(","),
                map_res(digit1, str::parse),
            ),
            tag(")"),
        ),
        |(x, y)| Ok::<Expr, nom::error::Error<&str>>(Expr::Mul(x, y)),
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_mul,
        map(tag("do()"), |_| Expr::Do),
        map(tag("don't()"), |_| Expr::Dont),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Expr>> {
    fold_many1(
        many_till(anychar, parse_expr),
        Vec::new,
        |mut acc: Vec<_>, (_, expr)| {
            acc.push(expr);
            acc
        },
    )(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (_, exprs) = parse_input(input).expect("input is incorrect");

    exprs
        .iter()
        .map(|expr| match expr {
            Expr::Mul(x, y) => x * y,
            _ => 0,
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let (_, exprs) = parse_input(input).expect("input is incorrect");

    exprs
        .iter()
        .scan(true, |enabled, expr| match expr {
            Expr::Mul(x, y) if *enabled => Some(x * y),
            Expr::Mul(_, _) => Some(0),
            Expr::Do => {
                *enabled = true;
                Some(0)
            }
            Expr::Dont => {
                *enabled = false;
                Some(0)
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn it_parses_single_mul_expr() {
        let mul = parse_mul("mul(2,4)");
        assert!(matches!(mul, Ok((_, Expr::Mul(x, y))) if x == 2 && y == 4));
    }

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(EXAMPLE_INPUT_1);
        assert_eq!(answer, 161);
    }

    #[test]
    fn it_solves_part2() {
        let answer = solve_part2(EXAMPLE_INPUT_2);
        assert_eq!(answer, 48);
    }
}
