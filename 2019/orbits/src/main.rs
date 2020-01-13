use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

fn run_a(edges: &HashMap<&str, &str>) -> isize {
    let mut distances: HashMap<&str, isize> = HashMap::new();
    distances.insert("COM", 0);
    for v in edges.keys() {
        calc_distance(v, edges, &mut distances);
    }
    distances.values().sum()
}

fn run_b(edges: &HashMap<&str, &str>) -> isize {
    let mut distances: HashMap<&str, isize> = HashMap::new();
    distances.insert("COM", 0);
    let me = calc_distance("YOU", edges, &mut distances);
    let santa = calc_distance_b("SAN", edges, &mut distances);
    me + santa - 2
}

fn calc_distance_b<'a>(
    v: &'a str,
    edges: &HashMap<&'a str, &'a str>,
    distances: &mut HashMap<&'a str, isize>,
) -> isize {
    if let Some(d) = distances.get(v) {
        return -(*d as isize);
    }
    if let Some(next) = edges.get(v) {
        let dist = 1 + calc_distance_b(next, edges, distances);
        distances.insert(v, dist);
        return dist;
    }
    panic!("{} orbits nobody", v);
}

fn calc_distance<'a>(
    v: &'a str,
    edges: &HashMap<&'a str, &'a str>,
    distances: &mut HashMap<&'a str, isize>,
) -> isize {
    if let Some(d) = distances.get(v) {
        return *d;
    }
    if let Some(next) = edges.get(v) {
        let dist = 1 + calc_distance(next, edges, distances);
        distances.insert(v, dist);
        return dist;
    }
    panic!("{} orbits nobody", v);
}

fn main() {
    let path: String = args().nth(1).expect("no data path provided");
    let content = read_to_string(path).expect("bad input");
    let data: HashMap<&str, &str> = content
        .lines()
        .map(|line| {
            let mut parts = line.split(')');
            let p0 = parts.next();
            let p1 = parts.next();
            // For backtracing it is easier to store orbiting_p: orbited_p
            (p1.unwrap(), p0.unwrap())
        })
        .collect();
    let all_orbits = run_a(&data);
    let me_to_san = run_b(&data);
    println!("[A] Num of orbits: {}", all_orbits);
    println!("[B] Orbit transfers between me and santa: {}", me_to_san);
}
