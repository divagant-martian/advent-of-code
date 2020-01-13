use std::fs::read_to_string;
pub use Trick::*;

pub enum Trick {
    Cut(isize),
    Deal,
    DealInc(usize),
}

pub fn parse_tricks(path: &str) -> Vec<Trick> {
    let aux = read_to_string(path).unwrap();
    let aux = aux.lines();
    let mut tricks = Vec::with_capacity(100);
    for line in aux {
        if line.contains("increment") {
            let inc: usize = line.split_whitespace().last().unwrap().parse().unwrap();
            tricks.push(DealInc(inc));
        } else if line.contains("cut") {
            let n: isize = line.split_whitespace().last().unwrap().parse().unwrap();
            tricks.push(Cut(n));
        } else {
            tricks.push(Deal);
        }
    }
    tricks
}

fn inverse(a: usize, n: usize) -> usize {
    let a = a as isize;
    let n = n as isize;
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r.div_euclid(newr);
        let tmpt = t;
        let tmpr = r;
        t = newt;
        newt = tmpt - quotient * newt;
        r = newr;
        newr = tmpr - quotient * newr;
    }
    if r > 1 {
        panic!("not invertible {} mod {}", a, n);
    }
    if t < 0 {
        t = t + n;
    }
    t as usize
}

pub fn parse_inverse_tricks(path: &str, nc: usize) -> Vec<Trick> {
    let aux = read_to_string(path).unwrap();
    let aux = aux.lines();
    let mut tricks = Vec::with_capacity(100);
    for line in aux {
        if line.contains("increment") {
            let inc: usize = line.split_whitespace().last().unwrap().parse().unwrap();
            let inc_inverse = inverse(inc, nc);
            tricks.push(DealInc(inc_inverse));
        } else if line.contains("cut") {
            let n: isize = line.split_whitespace().last().unwrap().parse().unwrap();
            tricks.push(Cut(-n));
        } else {
            tricks.push(Deal);
        }
    }
    tricks.into_iter().rev().collect()
}
