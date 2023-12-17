use std::{
    fmt::{self, Display, Formatter},
    io::stdin,
};

#[derive(Clone, Debug, Default)]
struct Pattern {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lines: Vec<_> = self
            .rows
            .iter()
            .map(|row| format!("{:0>16b}", row))
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}

impl Pattern {
    fn find_vert_refl_point(&self, ignored: Option<u64>) -> Option<u64> {
        // print!("check vert reflection point ");
        Self::find_refl_point(&self.cols, ignored)
    }

    fn find_hor_refl_point(&self, ignored: Option<u64>) -> Option<u64> {
        // print!("check hor reflection point ");
        Self::find_refl_point(&self.rows, ignored)
    }

    fn find_refl_point(vec: &Vec<u64>, ignored: Option<u64>) -> Option<u64> {
        for i in 1..vec.len() {
            if ignored.is_some_and(|ignored| ignored == i as u64) {
                continue;
            }

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
                    // println!("found at {i}");
                    return Some(i.try_into().unwrap());
                }
            }
        }

        // println!("none");
        None
    }

    fn alternates(&self) -> AlternatePatternsIterator {
        AlternatePatternsIterator {
            pat: self.clone(),
            row: 0,
            col: 0,
        }
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

struct AlternatePatternsIterator {
    pat: Pattern,
    row: usize,
    col: usize,
}

impl Iterator for AlternatePatternsIterator {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.pat.cols.len() - 1 && self.row == self.pat.rows.len() - 1 {
            None
        } else {
            let mut new_pat = self.pat.clone();
            new_pat.rows[self.row] ^= 1 << self.col;
            new_pat.cols[self.col] ^= 1 << self.row;

            if self.col == self.pat.cols.len() - 1 {
                self.col = 0;
                self.row += 1;
            } else {
                self.col += 1;
            }

            // println!("original");
            // println!("{}", self.pat);
            // println!("alternate");
            // println!("{}", new_pat);
            Some(new_pat)
        }
    }
}

fn main() {
    let mut sum = 0;

    while let Some(pattern) = read_pattern() {
        let orig_hor_refl_point = pattern.find_hor_refl_point(None);
        let orig_vert_refl_point = pattern.find_vert_refl_point(None);

        sum += pattern
            .alternates()
            .find_map(|alt| match alt.find_hor_refl_point(orig_hor_refl_point) {
                Some(n) => Some(n * 100),
                None => match alt.find_vert_refl_point(orig_vert_refl_point) {
                    Some(n) => Some(n),
                    None => None,
                },
            })
            .expect("no valid alternate was found");
    }

    println!("answer = {sum}");
}
