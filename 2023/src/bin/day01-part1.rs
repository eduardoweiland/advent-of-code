use std::io;

fn main() {
    let mut sum: u64 = 0;

    for line in io::stdin().lines() {
        let mut line = line.unwrap();
        line.retain(|c| c.is_ascii_digit());

        let mut first_and_last = String::from(&line[..1]);
        first_and_last.push_str(&line[line.len() - 1..]);

        sum += first_and_last.parse::<u64>().unwrap();
    }

    println!("{}", sum);
}
