use crate::signal::Signal;

use super::SignalSet;

impl From<&[Signal]> for SignalSet {
    fn from(slice: &[Signal]) -> Self {
        let mut me = SignalSet::empty();
        for s in slice {
            me.insert(*s);
        }
        me
    }
}

impl From<&str> for SignalSet {
    fn from(s: &str) -> Self {
        let mut me = SignalSet::empty();
        for s in s.trim().chars().map(Signal::from) {
            me.insert(s);
        }
        me
    }
}
