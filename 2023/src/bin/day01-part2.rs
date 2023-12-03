use regex::Regex;
use std::io;

fn main() {
    let mut sum: u64 = 0;

    let re_first = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    // crate regex não suporta find a partir do fim, então isso é feito com reverse :)
    let re_last = Regex::new(r"[0-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno").unwrap();

    for line in io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let first = parse_num(find(&re_first, &line));
        let last = parse_num(&reverse(find(&re_last, &reverse(&line))));

        let mut first_and_last = first;
        first_and_last.push_str(&last);

        sum += first_and_last.parse::<u64>().unwrap();
    }

    println!("{}", sum);
}

fn find<'a>(re: &Regex, str: &'a str) -> &'a str {
    re.find(str).unwrap().as_str()
}

fn reverse(str: &str) -> String {
    str.chars().rev().collect()
}

fn parse_num(str: &str) -> String {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        str => str,
    }
    .to_string()
}
