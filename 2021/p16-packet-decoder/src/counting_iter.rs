pub struct CountingIter<T> {
    next_calls: usize,
    iter: T,
}

impl<T> CountingIter<T> {
    pub fn new(t: T) -> Self {
        CountingIter {
            next_calls: 0,
            iter: t,
        }
    }
}

pub trait CountingIterator<Item>: Iterator<Item = Item> {
    fn calls(&self) -> usize;
}

impl<T: Iterator<Item = Item>, Item: std::fmt::Debug> CountingIterator<Item> for CountingIter<T> {
    fn calls(&self) -> usize {
        self.next_calls
    }
}

impl<T: Iterator<Item = I>, I: std::fmt::Debug> Iterator for CountingIter<T> {
    type Item = <T as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_calls += 1;
        let item = self.iter.next();
        let item = dbg!((item.unwrap(), self.next_calls)).0;
        Some(item)
    }
}
