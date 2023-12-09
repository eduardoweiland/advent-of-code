use std::io::stdin;

fn read_numbers() -> Vec<f64> {
    stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect()
}

// d = distance / tr = time of race / th = time hold
// d = (tr - th) * th
// d = tr*th - th²
// th² - tr*th + d = 0
// bhaskara :)
fn main() {
    let times = read_numbers();
    let distances = read_numbers();
    let mut ways_to_win = 1.0;

    for (tr, d) in times.iter().zip(distances.iter()) {
        let delta = tr.powf(2.0) - 4.0 * d;
        let x1 = (tr - delta.sqrt()) / 2.0;
        let x2 = (tr + delta.sqrt()) / 2.0;

        let min_hold = x1.floor() + 1.0;
        let max_hold = x2.ceil() - 1.0;
        ways_to_win *= max_hold - min_hold + 1.0;
    }

    println!("product of ways to win = {}", ways_to_win);
}
