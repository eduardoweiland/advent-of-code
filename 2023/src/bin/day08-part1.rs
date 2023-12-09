use std::{collections::HashMap, io::stdin};

fn main() {
    let instructions: Vec<char> = stdin().lines().next().unwrap().unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    stdin().lines().skip(1).for_each(|line| {
        let mut line = line.unwrap();
        line.retain(|c| c.is_ascii_uppercase());

        map.insert(
            line[0..3].to_string(),
            (line[3..6].to_string(), line[6..9].to_string()),
        );
    });

    let mut steps = 0;
    let mut current = "AAA".to_string();

    loop {
        current = match instructions[steps % instructions.len()] {
            'L' => map[&current].0.clone(),
            'R' => map[&current].1.clone(),
            _ => unreachable!(),
        };

        steps += 1;

        if current == "ZZZ" {
            break;
        }
    }

    println!("steps = {}", steps);
}
