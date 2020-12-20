use crate::Tile;

#[derive(Debug)]
pub enum Border {
    U,
    D,
    R,
    L,
}
pub const BORDERS: [Border; 4] = [Border::R, Border::L, Border::U, Border::D];

impl Border {
    #[allow(clippy::ptr_arg)]
    pub fn get(&self, tile: &Tile) -> Vec<usize> {
        let max_x = tile.iter().map(|p| p.0).max().unwrap();
        let max_y = tile.iter().map(|p| p.1).max().unwrap();
        let mut border: Vec<usize> = tile
            .iter()
            .filter(|(x, y)| match self {
                Border::U => *y == 0,
                Border::D => *y == max_y,
                Border::L => *x == 0,
                Border::R => *x == max_x,
            })
            .map(|(x, y)| match self {
                Border::U | Border::D => x,
                Border::L | Border::R => y,
            })
            .cloned()
            .collect();
        border.sort_unstable();
        border
    }

    pub fn oposite(&self) -> Self {
        match self {
            Border::U => Border::D,
            Border::D => Border::U,
            Border::R => Border::L,
            Border::L => Border::R,
        }
    }

    pub fn position_at_border(&self, placement: &(usize, usize)) -> (usize, usize) {
        match self {
            Border::U => (placement.0, placement.1 - 1),
            Border::D => (placement.0, placement.1 + 1),
            Border::R => (placement.0 + 1, placement.1),
            Border::L => (placement.0 - 1, placement.1),
        }
    }
}
