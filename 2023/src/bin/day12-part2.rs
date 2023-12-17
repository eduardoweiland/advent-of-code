use std::{io::stdin, cmp::Ordering::{Equal, Greater}};
use rayon::prelude::*;

#[memoize::memoize]
fn count_arrangements(springs: Vec<u8>, groups: Vec<u8>) -> u64 {
    match springs.get(0) {
        Some(b'.') => count_arrangements(springs[1..].to_vec(), groups),
        Some(b'?') => {
            let mut springs_with_hash = springs.clone();
            springs_with_hash[0] = b'#';
            let mut res = count_arrangements(springs_with_hash, groups.clone());
            res += count_arrangements(springs[1..].to_vec(), groups);
            res
        },
        Some(b'#') if groups.len() > 0 => {
            let len = groups[0] as usize;

            match springs.len().cmp(&len) {
                Equal if springs.iter().all(|c| *c != b'.') && groups.len() == 1 => 1,
                Greater if springs[0..len].iter().all(|c| *c != b'.') && springs[len] != b'#' => {
                    count_arrangements(springs[len + 1..].to_vec(), groups[1..].to_vec())
                },
                _ => 0,
            }
        },
        None if groups.len() == 0 => 1,
        _ => 0,
    }
}

fn parse_line_and_count(line: &mut str) -> u64 {
    let mut iter = line.split([' ', ',']);
    let orig_springs: Vec<u8> = iter.next().unwrap().bytes().collect();
    let orig_groups: Vec<u8> = iter.map(|n| n.parse().unwrap()).collect();

    let mut springs = vec![];
    let mut groups = vec![];

    for i in 0..5 {
        if i > 0 {
            springs.push(b'?');
        }
        springs.append(&mut orig_springs.clone());

        groups.append(&mut orig_groups.clone());
    }

    let res = count_arrangements(springs, groups);
    println!("{line} = {res}");

    res
}

fn main() {
    let mut lines: Vec<_> = stdin().lines().map(|line| line.unwrap()).collect();
    let sum: u64 = lines.par_iter_mut().map(|line| parse_line_and_count(line.as_mut_str())).sum();
    println!("sum = {sum}");
}
