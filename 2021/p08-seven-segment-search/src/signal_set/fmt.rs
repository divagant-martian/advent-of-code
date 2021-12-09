use super::SignalSet;

use crate::signal::Signal::*;

impl std::fmt::Display for SignalSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ' '.fmt(f)?;

        for s in [A, B, D, C, E, G, F] {
            if self.contains(s) {
                s.as_char().fmt(f)?;
            } else {
                ' '.fmt(f)?;
            }

            if s.enter() {
                '\n'.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for SignalSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
