use std::collections::HashSet;
use std::env;

fn get_points(line: &str) -> Vec<(isize, isize)> {
    let mut points = vec![];
    let mut cp: (isize, isize) = (0, 0);
    // Instruction components: a letter and a number (like R142)
    let instructions = line.split(',').map(|s| s.split_at(1)).map(|(d, a)| {
        (
            d.chars().nth(0).unwrap(),
            i32::from_str_radix(a, 10).unwrap(),
        )
    });
    for (dir, amount) in instructions {
        let (cx, cy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        for _ in 1..=amount {
            cp = (cp.0 + cx, cp.1 + cy);
            points.push(cp);
        }
    }
    points
}

fn get_intersection<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: std::cmp::PartialEq,
    T: Copy,
    T: std::cmp::Eq,
    T: std::hash::Hash,
{
    a.iter()
        .cloned()
        .collect::<HashSet<T>>()
        .intersection(&b.iter().cloned().collect::<HashSet<T>>())
        .cloned()
        .collect()
}

fn get_min_distance(
    points: &[(isize, isize)],
    path_a: &[(isize, isize)],
    path_b: &[(isize, isize)],
) -> usize {
    points
        .iter()
        .map(|p| {
            2 + path_a.iter().position(|x| p == x).unwrap()
                + path_b.iter().position(|x| p == x).unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let a = get_points(&args[1]);
    let b = get_points(&args[2]);
    let inter = get_intersection(&a, &b);

    // First challenge (Day 3)
    // println!("{:?}", inter.iter().map(|(x, y)| x.abs() + y.abs()).min());

    // Second challenge (Day 3)
    println!("{}", get_min_distance(&inter, &a, &b))
}
