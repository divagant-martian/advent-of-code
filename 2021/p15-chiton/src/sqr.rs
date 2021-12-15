#[derive(PartialEq, Eq, Debug)]
pub struct Sqr<T, const N: usize>([[T; N]; N]);

impl<T, const N: usize> std::ops::Deref for Sqr<T, N> {
    type Target = [[T; N]; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> Sqr<T, N> {
    pub const fn new(array: [[T; N]; N]) -> Self {
        Self(array)
    }
}

impl<T, const N: usize> std::ops::Index<(usize, usize)> for Sqr<T, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.0[y][x]
    }
}

impl<T, const N: usize> std::ops::IndexMut<(usize, usize)> for Sqr<T, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (y, x) = index;
        &mut self.0[y][x]
    }
}

impl<T: Default + Copy, const N: usize> Default for Sqr<T, N> {
    fn default() -> Self {
        Self([[T::default(); N]; N])
    }
}

impl<T, const N: usize> std::ops::Index<usize> for Sqr<T, N> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
