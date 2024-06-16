use std::io::stdin;

// https://math.blogoverflow.com/2014/06/04/greens-theorem-and-area-of-polygons/
// https://www.reddit.com/r/adventofcode/comments/18l0qtr/comment/kduuicl/
fn main() {
    let res = stdin()
        .lines()
        .map(|line| {
            let hexcode = line.as_ref().unwrap().trim_start_matches(|c| c != '#');
            let len = i64::from_str_radix(&hexcode[1..6], 16).unwrap();
            let dir = i64::from_str_radix(&hexcode[6..7], 10).unwrap();
            (dir, len)
        })
        .collect::<Vec<_>>()
        .iter()
        .rfold((0, 0, 0, 0), |(x, y, area, perimeter), (dir, len)| {
            let (next_x, next_y) = match dir {
                0 => (x + len, y),
                1 => (x, y - len),
                2 => (x - len, y),
                3 => (x, y + len),
                _ => unreachable!("invalid direction in input"),
            };

            let area = area + (next_x + x) * (next_y - y) / 2;
            (next_x, next_y, area, perimeter + len)
        });

    println!("{}", res.2 + res.3 / 2 + 1);
}
