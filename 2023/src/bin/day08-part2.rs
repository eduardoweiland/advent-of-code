use std::{collections::HashMap, io::stdin};

fn main() {
    let instructions: Vec<char> = stdin().lines().next().unwrap().unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut currents = vec![];

    stdin().lines().skip(1).for_each(|line| {
        let mut line = line.unwrap();
        line.retain(|c| c.is_ascii_uppercase() || c.is_ascii_digit());

        let key = line[0..3].to_string();

        map.insert(
            key.clone(),
            (line[3..6].to_string(), line[6..9].to_string()),
        );

        if key.ends_with('A') {
            currents.push(key.clone());
        }
    });

    let steps = currents
        .iter()
        .map(|current| {
            let mut steps = 0;
            let mut current = current.clone();

            loop {
                current = match instructions[steps % instructions.len()] {
                    'L' => map[&current].0.clone(),
                    'R' => map[&current].1.clone(),
                    _ => unreachable!(),
                };

                steps += 1;

                if current.ends_with('Z') {
                    break;
                }
            }

            steps
        })
        .reduce(num::integer::lcm)
        .unwrap();

    println!("steps = {}", steps);
}
