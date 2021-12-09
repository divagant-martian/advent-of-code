use crate::signal::Signal;

mod convert;
mod fmt;
mod iter;
mod ops;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SignalSet([bool; 7]);

impl SignalSet {
    pub fn empty() -> Self {
        Self([false; 7])
    }

    pub fn contains(&self, signal: Signal) -> bool {
        self.0[usize::from(signal as u8)]
    }

    pub fn len(&self) -> usize {
        self.0.iter().filter(|&s| *s).count()
    }

    pub fn insert(&mut self, signal: Signal) {
        self.0[usize::from(signal as u8)] = true;
    }

    pub fn remove(&mut self, signal: Signal) {
        self.0[usize::from(signal as u8)] = false;
    }

    pub fn iter(&self) -> impl Iterator<Item = Signal> + '_ {
        Signal::ALL
            .into_iter()
            .filter(|&&s| self.contains(s))
            .cloned()
    }

    pub fn is_superset(&self, other: SignalSet) -> bool {
        other.iter().all(|s| self.contains(s))
    }
}
