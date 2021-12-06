use std::io;

pub fn get_total_of_fishes_after_some_days(mut fishes: Vec<u64>, days: u32) -> u64 {
    for _i in 0..days {
        let fishes_at_zero = fishes.remove(0);
        fishes[6] += fishes_at_zero;
        fishes.push(fishes_at_zero);
    }

    return fishes.into_iter().reduce(|total, count| total + count).unwrap();
}

pub fn create_fishes_from_stdin() -> Vec<u64> {
    let mut fishes_by_timer = vec![0; 9];

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut timers = line.trim().split(',');

    while let Some(timer) = timers.next() {
        fishes_by_timer[timer.parse::<usize>().unwrap()] += 1;
    }

    return fishes_by_timer;
}
