use regex::{CaptureMatches, Regex};
use std::io;

fn main() {
    let mut sum: u64 = 0;
    let re_blocks = Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();
    let re_game_id = Regex::new(r"^Game (\d+):").unwrap();

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        if is_game_possible(re_blocks.captures_iter(&line)) {
            sum += re_game_id
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
        }
    }

    println!("{}", sum);
}

fn is_game_possible(captures: CaptureMatches) -> bool {
    for capture in captures {
        let count: u64 = capture.name("count").unwrap().as_str().parse().unwrap();

        let limit = match capture.name("color").unwrap().as_str() {
            "red" => 12,
            "green" => 13,
            "blue" => 14,
            _ => unreachable!(),
        };

        if count > limit {
            return false;
        }
    }

    true
}
