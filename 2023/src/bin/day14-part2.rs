use std::io::stdin;

#[memoize::memoize]
fn tilt_north(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();

    for col in 0..map[0].len() {
        let mut last_occupied_row: isize = -1;

        for row in 0..map.len() {
            match map[row][col] {
                '#' => last_occupied_row = row as isize,
                'O' => {
                    last_occupied_row += 1;

                    if last_occupied_row != row as isize {
                        map[last_occupied_row as usize][col] = 'O';
                        map[row][col] = '.';
                    }
                }
                _ => (),
            }
        }
    }

    map
}

#[memoize::memoize]
fn tilt_south(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();

    for col in (0..map[0].len()).rev() {
        let mut last_occupied_row = map.len();

        for row in (0..map.len()).rev() {
            match map[row][col] {
                '#' => last_occupied_row = row,
                'O' => {
                    last_occupied_row -= 1;

                    if last_occupied_row != row {
                        map[last_occupied_row][col] = 'O';
                        map[row][col] = '.';
                    }
                }
                _ => (),
            }
        }
    }

    map
}

#[memoize::memoize]
fn tilt_west(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();

    for row in 0..map.len() {
        let mut last_occupied_col: isize = -1;

        for col in 0..map[row].len() {
            match map[row][col] {
                '#' => last_occupied_col = col as isize,
                'O' => {
                    last_occupied_col += 1;

                    if last_occupied_col != col as isize {
                        map[row][last_occupied_col as usize] = 'O';
                        map[row][col] = '.';
                    }
                }
                _ => (),
            }
        }
    }

    map
}

#[memoize::memoize]
fn tilt_east(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();

    for row in (0..map.len()).rev() {
        let mut last_occupied_col = map.len();

        for col in (0..map[row].len()).rev() {
            match map[row][col] {
                '#' => last_occupied_col = col,
                'O' => {
                    last_occupied_col -= 1;

                    if last_occupied_col != col {
                        map[row][last_occupied_col] = 'O';
                        map[row][col] = '.';
                    }
                }
                _ => (),
            }
        }
    }

    map
}

fn calc_load(map: &Vec<Vec<char>>) -> u64 {
    let mut load: u64 = 0;

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'O' {
                load += (map.len() - row) as u64;
            }
        }
    }

    load
}

fn tilt_cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let map = tilt_north(map);
    let map = tilt_west(map);
    let map = tilt_south(map);
    let map = tilt_east(map);
    map
}

fn main() {
    let mut map: Vec<Vec<char>> = stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut maps_after_each_cycle: Vec<Vec<Vec<char>>> = vec![];
    let mut cycles_to_repeat = 0;
    let mut first_repeated_map = 0;

    loop {
        map = tilt_cycle(map);

        if let Some(repeat) = maps_after_each_cycle.iter().position(|m| *m == map) {
            println!("found repeat after {} eq {}", cycles_to_repeat, repeat);
            cycles_to_repeat -= repeat;
            first_repeated_map = repeat;
            break;
        } else {
            maps_after_each_cycle.push(map.clone());
            cycles_to_repeat += 1;
        }
    }

    let repetitions = (1_000_000_000 - first_repeated_map) / cycles_to_repeat;
    let remaining_cycles = 1_000_000_000 - repetitions * cycles_to_repeat - first_repeated_map;
    let last_map = &maps_after_each_cycle[first_repeated_map + remaining_cycles - 1];
    println!("repetitions = {repetitions}");
    println!("remaining_cycles = {remaining_cycles}");

    let load = calc_load(last_map);
    println!("answer = {load}");
}
