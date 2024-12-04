use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Mul(u32, u32);

fn parse_mul(input: &str) -> IResult<&str, Mul> {
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
        |(x, y)| Ok::<Mul, nom::error::Error<&str>>(Mul(x, y)),
    )(input)
}

fn discard_not_mul(input: &str) -> IResult<&str, ()> {
    let mut rest = input;

    loop {
        if rest.is_empty() {
            break;
        }

        match parse_mul(rest) {
            Ok(_) => break,
            Err(_) => rest = &rest[1..],
        }
    }

    Ok((rest, ()))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Mul>> {
    all_consuming(many0(delimited(
        discard_not_mul,
        parse_mul,
        discard_not_mul,
    )))(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    match parse_input(input) {
        Ok((_, exprs)) => exprs.iter().map(|Mul(x, y)| x * y).sum(),
        Err(err) => panic!("{}", err),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn it_parses_single_mul_expr() {
        let mul = parse_mul("mul(2,4)");
        assert!(matches!(mul, Ok((_, Mul(x, y))) if x == 2 && y == 4));
    }

    #[test]
    fn it_discards_not_mul_expr() {
        let mul = discard_not_mul("xmul(2,4)");
        assert!(matches!(mul, Ok((rest, _)) if rest == "mul(2,4)"));
    }

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(EXAMPLE_INPUT);
        assert_eq!(answer, 161);
    }
}
