use std::{
    cmp::Ordering::{Equal, Greater},
    io::stdin,
};

fn count_arrangements(springs: &mut [u8], groups: &[u8]) -> u64 {
    match springs.get(0) {
        Some(b'.') => count_arrangements(&mut springs[1..], groups),
        Some(b'?') => {
            springs[0] = b'#';
            let mut res = count_arrangements(springs, groups);
            res += count_arrangements(&mut springs[1..], groups);
            springs[0] = b'?';
            res
        }
        Some(b'#') if groups.len() > 0 => {
            let len = groups[0] as usize;

            match springs.len().cmp(&len) {
                Equal if springs.iter().all(|c| *c != b'.') && groups.len() == 1 => 1,
                Greater if springs[0..len].iter().all(|c| *c != b'.') && springs[len] != b'#' => {
                    count_arrangements(&mut springs[len + 1..], &groups[1..])
                }
                _ => 0,
            }
        }
        None if groups.len() == 0 => 1,
        _ => 0,
    }
}

fn parse_line_and_count(line: &mut str) -> u64 {
    let mut iter = line.split([' ', ',']);
    let mut springs: Vec<u8> = iter.next().unwrap().bytes().collect();
    let groups: Vec<u8> = iter.map(|n| n.parse().unwrap()).collect();

    let res = count_arrangements(&mut springs, &groups);
    println!("{line} = {res}");

    res
}

fn main() {
    let sum: u64 = stdin()
        .lines()
        .map(|line| parse_line_and_count(line.unwrap().as_mut_str()))
        .sum();
    println!("sum = {sum}");
}
