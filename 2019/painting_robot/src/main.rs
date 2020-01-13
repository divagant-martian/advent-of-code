use painting_robot::doshit;

fn main() {
    let ans_a = doshit(0).len();
    println!("the robot painted {} cells", ans_a);

    let painted_cells = doshit(1);
    let (x_max, x_min, y_max, y_min) =
        painted_cells
            .keys()
            .fold((0, 0, 0, 0), |(x_max, x_min, y_max, y_min), &(x, y)| {
                (x_max.max(x), x_min.min(x), y_max.max(y), y_min.min(y))
            });

    for y in (y_min..=y_max).rev() {
        let mut row = String::new();
        for x in x_min..=x_max {
            match painted_cells.get(&(x, y)).cloned().unwrap_or_default() {
                0 => {
                    row.push(' ');
                    row.push(' ');
                }
                _ => {
                    row.push('█');
                    row.push('█');
                }
            }
        }
        println!("{}", row);
    }
}
