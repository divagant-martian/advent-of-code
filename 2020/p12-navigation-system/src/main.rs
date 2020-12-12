use std::fs::read_to_string;
enum Ins {
    N(f64),
    S(f64),
    E(f64),
    W(f64),
    L(f64),
    R(f64),
    F(f64),
}
fn load_instructions(filename: &'static str) -> Vec<Ins> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            let num = l[1..].parse::<f64>().unwrap();
            match &l[..1] {
                "N" => Ins::N(num),
                "S" => Ins::S(num),
                "E" => Ins::E(num),
                "W" => Ins::W(num),
                "L" => Ins::L(2_f64 * std::f64::consts::PI * num / 360_f64),
                "R" => Ins::R(2_f64 * std::f64::consts::PI * num / 360_f64),
                "F" => Ins::F(num),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    let data = "data/chris.txt";
    let (final_x, final_y, _final_angle) =
        load_instructions(data)
            .iter()
            .fold((0_f64, 0_f64, 0_f64), |(x, y, angle), ins| {
                let end = match ins {
                    Ins::N(n) => (x, y + n, angle),
                    Ins::S(n) => (x, y - n, angle),
                    Ins::E(n) => (x + n, y, angle),
                    Ins::W(n) => (x - n, y, angle),
                    Ins::L(n) => (x, y, angle + n),
                    Ins::R(n) => (x, y, angle - n),
                    Ins::F(n) => ((n * angle.cos()) + x, (n * angle.sin()) + y, angle),
                };
                end
            });

    println!("PART 1! {}", (final_x.abs() + final_y.abs()).round());

    let f = load_instructions(data).iter().fold(
        (10_f64, 1_f64, 0_f64, 0_f64),
        |(x, y, ship_x, ship_y), ins| {
            let end = match ins {
                Ins::N(n) => (x, y + n, ship_x, ship_y),
                Ins::S(n) => (x, y - n, ship_x, ship_y),
                Ins::E(n) => (x + n, y, ship_x, ship_y),
                Ins::W(n) => (x - n, y, ship_x, ship_y),
                Ins::L(n) => {
                    let angle_0 = y.atan2(x) + n;
                    let r = (x.powi(2) + y.powi(2)).sqrt();
                    (r * angle_0.cos(), r * angle_0.sin(), ship_x, ship_y)
                }
                Ins::R(n) => {
                    let angle_0 = y.atan2(x) - n;
                    let r = (x.powi(2) + y.powi(2)).sqrt();
                    (r * angle_0.cos(), r * angle_0.sin(), ship_x, ship_y)
                }
                Ins::F(n) => (x, y, ship_x + n * x, ship_y + n * y),
            };
            end
        },
    );

    println!("PART 2! {}", (f.2.abs() + f.3.abs()).round());
}
