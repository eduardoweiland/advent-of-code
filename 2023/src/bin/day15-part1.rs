use std::io::stdin;

fn calc_hash(input: &str) -> u64 {
    let mut hash = 0;

    input.bytes().for_each(|b| {
        hash += b as u64;
        hash *= 17;
        hash %= 256;
    });

    hash
}

fn main() {
    println!(
        "anwser = {}",
        stdin()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(calc_hash)
            .sum::<u64>()
    );
}
