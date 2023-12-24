use std::io::stdin;

fn tilt_north(map: &mut Vec<Vec<char>>) {
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

fn main() {
    let mut map: Vec<Vec<char>> = stdin().lines().map(|l| l.unwrap().chars().collect()).collect();

    tilt_north(&mut map);
    let load = calc_load(&map);
    println!("answer = {load}");
}
