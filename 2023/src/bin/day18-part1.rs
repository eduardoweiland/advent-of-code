use std::io::stdin;

// https://math.blogoverflow.com/2014/06/04/greens-theorem-and-area-of-polygons/
// https://www.reddit.com/r/adventofcode/comments/18l0qtr/comment/kduuicl/
fn main() {
    let res = stdin()
        .lines()
        .map(|line| {
            let mut iter = line.as_ref().unwrap().split_whitespace();
            let dir = iter.next().unwrap().chars().next().unwrap();
            let len: i32 = iter.next().unwrap().parse().unwrap();
            (dir, len)
        })
        .collect::<Vec<_>>()
        .iter()
        .rfold((0, 0, 0, 0), |(x, y, area, perimeter), (dir, len)| {
            let (next_x, next_y) = match dir {
                'R' => (x + len, y),
                'L' => (x - len, y),
                'U' => (x, y + len),
                'D' => (x, y - len),
                _ => unreachable!("invalid direction in input"),
            };

            let area = area + (next_x + x) * (next_y - y) / 2;
            (next_x, next_y, area, perimeter + len)
        });

    println!("{}", res.2 + res.3 / 2 + 1);
}
