use std::io::stdin;

#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn distance_to(&self, other: &Galaxy) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Debug, Default)]
struct Universe {
    galaxies: Vec<Galaxy>,
    rows: usize,
    cols: usize,
}

impl Universe {
    fn from_stdin(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Self {
        let mut universe = Self::default();

        lines.enumerate().for_each(|(row, line)| {
            let line = line.unwrap();

            universe.rows += 1;
            universe.cols = line.len();

            line.char_indices().for_each(|(col, char)| {
                if char == '#' {
                    universe.galaxies.push(Galaxy { row, col });
                }
            });
        });

        universe
    }

    fn expand(&mut self) {
        self.expand_vertically();
        self.expand_horizontally();
    }

    fn expand_vertically(&mut self) {
        let occupied_rows: Vec<_> = self.galaxies.iter().map(|g| g.row).collect();
        let empty_rows: Vec<_> = (0..self.rows)
            .filter(|r| !occupied_rows.contains(r))
            .collect();

        self.galaxies.iter_mut().for_each(|galaxy| {
            galaxy.row += empty_rows.iter().filter(|r| **r < galaxy.row).count() * (1000000 - 1);
        });
    }

    fn expand_horizontally(&mut self) {
        let occupied_cols: Vec<_> = self.galaxies.iter().map(|g| g.col).collect();
        let empty_cols: Vec<_> = (0..self.cols)
            .filter(|r| !occupied_cols.contains(r))
            .collect();

        self.galaxies.iter_mut().for_each(|galaxy| {
            galaxy.col += empty_cols.iter().filter(|r| **r < galaxy.col).count() * (1000000 - 1);
        });
    }
}

fn main() {
    let mut universe = Universe::from_stdin(&mut stdin().lines());
    universe.expand();
    let mut sum_of_distances = 0;

    for (index, galaxy) in universe.galaxies.iter().enumerate().skip(1) {
        for other_galaxy in universe.galaxies[0..index].iter() {
            let distance = galaxy.distance_to(other_galaxy);
            // println!("distance between {:?} and {:?} = {}", galaxy, other_galaxy, distance);
            sum_of_distances += distance;
        }
    }

    println!("sum of distances = {}", sum_of_distances);
}
