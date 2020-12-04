use std::fs::read_to_string;

fn count_for_slope(
    trees: &[(usize, usize)],
    width: usize,
    slope_x0: usize,
    slope_y0: usize,
) -> usize {
    trees
        .iter()
        .filter(|(x, y)| {
            // let cond = would_encounter;
            let first = y * slope_x0;
            let second = slope_y0 * x;
            if (first >= second) && ((first - second) % (slope_y0 * width) == 0) {
                // println!("{}, {}", x, y);
                true
            } else {
                false
            }
        })
        .count()
}

fn get_trees(file_name: &str) -> (Vec<(usize, usize)>, usize) {
    let d = read_to_string(file_name).expect("bad input");
    let mut width = None;
    let mut x = 0;
    let mut y = 0;
    let mut trees = Vec::new();
    for c in d.chars() {
        match c {
            '#' => {
                trees.push((x, y));
                x += 1;
            }
            '.' => x += 1,
            '\n' => {
                if width.is_none() {
                    width = Some(x);
                }
                x = 0;
                y += 1;
            }
            _ => unreachable!("bad char"),
        }
    }
    (trees, width.unwrap())
}

fn main() {
    let (trees, width) = get_trees("data/input1.txt");
    let mut ans = 1;
    for (slope_x0, slope_y0) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
    // for (slope_x0, slope_y0) in &[(1, 4)] {
        let count = count_for_slope(&trees, width, *slope_x0, *slope_y0);
        println!("slope ({}, {}) has count {}", slope_x0, slope_y0, count);
        ans *= count;
    }
    println!("ans: {}", ans);
}
