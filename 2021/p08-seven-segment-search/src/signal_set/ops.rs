use std::ops::{BitOr, BitOrAssign, BitXorAssign, Not, Sub, SubAssign};

use super::SignalSet;

/// Union
impl BitOrAssign for SignalSet {
    fn bitor_assign(&mut self, rhs: Self) {
        for (i, rhs) in rhs.0.into_iter().enumerate() {
            self.0[i] |= rhs;
        }
    }
}

impl BitOr for SignalSet {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

/// Sym diff
impl BitXorAssign for SignalSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (i, rhs) in rhs.0.into_iter().enumerate() {
            self.0[i] ^= rhs;
        }
    }
}

/// Diff
impl SubAssign for SignalSet {
    fn sub_assign(&mut self, rhs: Self) {
        for (i, rhs) in rhs.0.into_iter().enumerate() {
            self.0[i] &= !rhs;
        }
    }
}

impl Sub for SignalSet {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

/// Complement
impl Not for SignalSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        SignalSet(self.0.map(Not::not))
    }
}
