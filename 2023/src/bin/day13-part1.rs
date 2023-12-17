use std::io::stdin;

#[derive(Debug, Default)]
struct Pattern {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl Pattern {
    fn find_vert_refl_point(&self) -> Option<u64> {
        Self::find_refl_point(&self.cols)
    }

    fn find_hor_refl_point(&self) -> Option<u64> {
        Self::find_refl_point(&self.rows)
    }

    fn find_refl_point(vec: &Vec<u64>) -> Option<u64> {
        for i in 1..vec.len() {
            if vec[i - 1] == vec[i] {
                // check if reflections match
                let (mut j, mut k) = (i - 1, i);
                let mut is_reflection = true;

                while j > 0 && k < vec.len() - 1 {
                    if vec[j - 1] != vec[k + 1] {
                        is_reflection = false;
                        break;
                    } else {
                        j -= 1;
                        k += 1;
                    }
                }

                if is_reflection {
                    return Some(i.try_into().unwrap());
                }
            }
        }

        None
    }
}

fn read_pattern() -> Option<Pattern> {
    let mut pattern = Pattern::default();
    let mut row = 0;

    while let Some(Ok(line)) = stdin().lines().next() {
        if line.is_empty() {
            break;
        }

        pattern.rows.push(0);

        line.char_indices().for_each(|(col, char)| {
            if row == 0 {
                pattern.cols.push(0);
            }

            if char == '#' {
                pattern.rows[row] |= 1 << col;
                pattern.cols[col] |= 1 << row;
            }
        });

        row += 1;
    }

    if pattern.rows.len() > 0 {
        Some(pattern)
    } else {
        None
    }
}

fn main() {
    let mut sum = 0;

    while let Some(pattern) = read_pattern() {
        sum += match pattern.find_vert_refl_point() {
            Some(n) => n,
            None => match pattern.find_hor_refl_point() {
                Some(n) => n * 100,
                None => unreachable!("no reflection found"),
            },
        };
    }

    println!("answer = {sum}");
}
