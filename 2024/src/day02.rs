use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count() as i32
}

#[aoc(day2, part2)]
pub fn solve_part2(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter(|report| {
            if is_safe_report(report) {
                true
            } else {
                // brute-force time
                for i in 0..report.len() {
                    let mut modified_report = report.to_vec();
                    modified_report.remove(i);
                    if is_safe_report(&modified_report) {
                        return true;
                    }
                }

                false
            }
        })
        .count() as i32
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let last = report.iter();
    let next = report.iter().skip(1);

    last.zip(next)
        .into_iter()
        .map(|(last, next)| last - next)
        .scan(0, |last_diff, curr_diff| {
            if curr_diff > 3 || curr_diff < -3 || curr_diff == 0 {
                Some(false)
            } else if *last_diff > 0 && curr_diff < 0 {
                Some(false)
            } else if *last_diff < 0 && curr_diff > 0 {
                Some(false)
            } else {
                *last_diff = curr_diff;
                Some(true)
            }
        })
        .all(|safe| safe)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn it_parses_input() {
        let reports = parse_input(EXAMPLE_INPUT);
        assert_eq!(reports, [
            [7, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
        ]);
    }

    #[test]
    fn it_solves_part1() {
        let answer = solve_part1(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 2);
    }

    #[test]
    fn it_solves_part2() {
        let answer = solve_part2(&parse_input(EXAMPLE_INPUT));
        assert_eq!(answer, 4);
    }
}
