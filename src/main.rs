use std::fs::read_to_string;

trait Shuffle {
    fn deal(&mut self);
    fn cut(&mut self, n: isize);
    fn deal_inc(&mut self, inc: usize);
}

type Deck = [usize];
impl Shuffle for Deck {
    fn deal(&mut self) {
        self.reverse()
    }

    fn cut(&mut self, n: isize) {
        if n >= 0 {
            self.rotate_left(n as usize);
        } else {
            self.rotate_right(n.abs() as usize);
        }
    }

    fn deal_inc(&mut self, inc: usize) {
        let nc = self.len();
        let bkp = self.to_owned();
        for i in 0..nc {
            let new_i = (i * inc).rem_euclid(nc);
            self[new_i] = bkp[i];
        }
    }
}

fn part1() {
    let nc = 10006;
    let mut deck = (0..=nc).collect::<Vec<usize>>();
    let aux = read_to_string("data/input.txt").unwrap();
    for line in aux.lines() {
        if line.contains("increment") {
            let inc: usize = line.split_whitespace().last().unwrap().parse().unwrap();
            deck.deal_inc(inc);
        } else if line.contains("cut") {
            let n: isize = line.split_whitespace().last().unwrap().parse().unwrap();
            deck.cut(n);
        } else {
            deck.deal();
        }
    }
    let mut i = 0;
    for n in deck {
        if n == 2019 {
            println!("Part 1 {}", i);
            break;
        }
        i += 1;
    }
}

// -----------------------------PART 2-------------------------------
fn deal_sim(nc: usize, index: usize) -> usize {
    // println!("deal_sim");
    nc - 1 - index
}

fn cut_sim(nc: usize, index: usize, n: isize) -> usize {
    // println!("cut_sim{}", n);
    if n >= 0 {
        let n = n as usize;
        if index >= n {
            index - n
        } else {
            nc - (n - index)
        }
    } else {
        let n = n.abs() as usize;
        if index < nc - n {
            index + n
        } else {
            (index + n).rem_euclid(nc)
        }
    }
}

fn deal_inc_sim(nc: usize, index: usize, inc: usize) -> usize {
    // println!("deal_inc_sim{}", inc);
    (index * inc).rem_euclid(nc)
}

fn main() {
    let aux = read_to_string("data/input.txt").unwrap();
    let nc = 119315717514047_usize;
    let nexec = 101741582076661_usize;
    // let nc = 10007;
    // let nexec = 1;
    //
    // let aux = read_to_string("data/test.txt").unwrap();
    // let nc = 10;
    // let nexec = 1;

    let mut executions: Vec<Box<dyn Fn(usize) -> usize>> = Vec::new();

    for line in aux.lines() {
        if line.contains("increment") {
            let inc: usize = line.split_whitespace().last().unwrap().parse().unwrap();
            let cl = move |i| deal_inc_sim(nc, i, inc);
            executions.push(Box::new(cl));
        } else if line.contains("cut") {
            let n: isize = line.split_whitespace().last().unwrap().parse().unwrap();
            let cl = move |i| cut_sim(nc, i, n);
            executions.push(Box::new(cl));
        } else {
            let cl = move |i| deal_sim(nc, i);
            executions.push(Box::new(cl));
        }
    }

    let one_round = |i| {
        let mut i = i;
        for function in &executions {
            i = function(i);
        }
        i
    };

    let nexec_rounds = |i| {
        let mut i = i;
        for _ in 0..nexec {
            i = one_round(i);
        }
        i
    };

    for i in 0..nc {
        if nexec_rounds(i) == 2020 {
            println!("{} produces 2020", i);
            break;
        }
    }
    // println!("{}", nexec_rounds(2019));
}
