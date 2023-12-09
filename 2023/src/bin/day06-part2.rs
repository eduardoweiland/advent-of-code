use std::io::stdin;

fn read_number() -> f64 {
    let mut line = stdin().lines().next().unwrap().unwrap();
    line.retain(|c| c.is_ascii_digit());
    line.parse().unwrap()
}

// d = distance / tr = time of race / th = time hold
// d = (tr - th) * th
// d = tr*th - th²
// th² - tr*th + d = 0
// bhaskara :)
fn main() {
    let tr = read_number();
    let d = read_number();

    let delta = tr.powf(2.0) - 4.0 * d;
    let x1 = (tr - delta.sqrt()) / 2.0;
    let x2 = (tr + delta.sqrt()) / 2.0;

    let min_hold = x1.floor() + 1.0;
    let max_hold = x2.ceil() - 1.0;
    let ways_to_win = max_hold - min_hold + 1.0;

    println!("ways to win = {}", ways_to_win);
}
