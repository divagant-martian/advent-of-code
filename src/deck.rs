use crate::shuffle_trick::Trick;

pub trait Shuffle {
    fn shuffle(&mut self, trick: &Trick);
}

pub type Deck = [usize];
impl Shuffle for Deck {
    fn shuffle(&mut self, trick: &Trick) {
        match trick {
            &Trick::Cut(n) => self.cut(n),
            &Trick::DealInc(inc) => self.deal_inc(inc),
            &Trick::Deal => self.deal(),
        }
    }
}

pub trait Shufflable {
    fn deal(&mut self);
    fn cut(&mut self, n: isize);
    fn deal_inc(&mut self, inc: usize);
}

impl Shufflable for Deck {
    fn deal(&mut self) {
        self.reverse();
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

#[derive(Debug)]
pub struct IndexSim {
    i: usize,
    nc: usize,
}

impl Shuffle for IndexSim {
    fn shuffle(&mut self, trick: &Trick) {
        match trick {
            &Trick::Cut(n) => self.cut(n),
            &Trick::DealInc(inc) => self.deal_inc(inc),
            &Trick::Deal => self.deal(),
        }
    }
}

impl IndexSim {
    pub fn new(i: usize, nc: usize) -> Self {
        IndexSim { i, nc }
    }
}

impl Shufflable for IndexSim {
    fn deal(&mut self) {
        self.i = self.nc - 1 - self.i;
    }

    fn cut(&mut self, n: isize) {
        self.i = if n >= 0 {
            let n = n as usize;
            if self.i >= n {
                self.i - n
            } else {
                self.nc - (n - self.i)
            }
        } else {
            let n = n.abs() as usize;
            if self.i < self.nc - n {
                self.i + n
            } else {
                (self.i + n).rem_euclid(self.nc)
            }
        };
    }

    fn deal_inc(&mut self, inc: usize) {
        self.i = ((self.i as u128 * inc as u128) as usize).rem_euclid(self.nc);
    }
}
