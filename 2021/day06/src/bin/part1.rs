fn main() {
    let fishes = day06::create_fishes_from_stdin();
    println!("{}", day06::get_total_of_fishes_after_some_days(fishes, 80));
}
