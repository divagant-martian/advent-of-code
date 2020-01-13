use jupiter_moons::{sim, step, Moon};

fn find_period(moons: &mut [Moon], to_find: &[Moon]) -> i64 {
    let mut i = 1;
    while {
        step(moons);
        !to_find.eq(moons)
    } {
        i += 1;
    }
    i
}

fn main() {
    // let m0 = (-8, -10, 0);
    // let m1 = (5, 5, 10);
    // let m2 = (2, -7, 3);
    // let m3 = (9, -8, -3);

    let m0 = (5, 4, 4);
    let m1 = (-11, -11, -3);
    let m2 = (0, 7, 0);
    let m3 = (-13, 2, 10);

    let mut moons = vec![
        Moon::new(m0.0, m0.1, m0.2),
        Moon::new(m1.0, m1.1, m1.2),
        Moon::new(m2.0, m2.1, m2.2),
        Moon::new(m3.0, m3.1, m3.2),
    ];
    sim(&mut moons, 1000);
    println!("{:?}", moons.iter().map(|m| m.total_energy()).sum::<i32>());
    let mut x_sim = vec![
        Moon::new(m0.0, 0, 0),
        Moon::new(m1.0, 0, 0),
        Moon::new(m2.0, 0, 0),
        Moon::new(m3.0, 0, 0),
    ];

    let mut y_sim = vec![
        Moon::new(0, m0.1, 0),
        Moon::new(0, m1.1, 0),
        Moon::new(0, m2.1, 0),
        Moon::new(0, m3.1, 0),
    ];

    let mut z_sim = vec![
        Moon::new(0, 0, m0.2),
        Moon::new(0, 0, m1.2),
        Moon::new(0, 0, m2.2),
        Moon::new(0, 0, m3.2),
    ];

    let x_orig = x_sim.clone();
    let y_orig = y_sim.clone();
    let z_orig = z_sim.clone();

    let x_period = find_period(&mut x_sim, &x_orig);
    let y_period = find_period(&mut y_sim, &y_orig);
    let z_period = find_period(&mut z_sim, &z_orig);
    println!(
        "https://www.wolframalpha.com/input/?i=lcm({}%2C+{}%2C+{})",
        x_period, y_period, z_period
    );
}
