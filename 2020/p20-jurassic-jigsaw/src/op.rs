use crate::Tile;

pub const OPERATIONS: [Op; 8] = [
    Op {
        rotate: None,
        flip: None,
    },
    Op {
        rotate: Some(Rotate::One),
        flip: None,
    },
    Op {
        rotate: Some(Rotate::Two),
        flip: None,
    },
    Op {
        rotate: Some(Rotate::Three),
        flip: None,
    },
    Op {
        rotate: None,
        flip: Some(Flip),
    },
    Op {
        rotate: Some(Rotate::One),
        flip: Some(Flip),
    },
    Op {
        rotate: Some(Rotate::Two),
        flip: Some(Flip),
    },
    Op {
        rotate: Some(Rotate::Three),
        flip: Some(Flip),
    },
];

#[derive(Debug)]
pub struct Flip;
#[derive(Debug)]
pub enum Rotate {
    One,
    Two,
    Three,
}

#[derive(Debug)]
pub struct Op {
    pub rotate: Option<Rotate>,
    pub flip: Option<Flip>,
}

pub trait Operation {
    #[allow(clippy::ptr_arg)]
    fn operate_clone(&self, tile: &Tile) -> Tile {
        let mut tile = tile.to_owned();
        self.operate(&mut tile);
        tile
    }
    fn operate(&self, tile: &mut Tile);
}

impl Operation for Flip {
    fn operate(&self, tile: &mut Tile) {
        let width = *tile.iter().map(|(x, _y)| x).max().unwrap();
        for (x, _) in tile.iter_mut() {
            *x = width - *x;
        }
    }
}

impl Operation for Rotate {
    fn operate(&self, tile: &mut Tile) {
        let times = match self {
            Rotate::One => 1,
            Rotate::Two => 2,
            Rotate::Three => 3,
        };
        for _ in 0..times {
            // rotate is transpose and then horizontal flip

            // transpose:
            for pos in tile.iter_mut() {
                *pos = (pos.1, pos.0);
            }
            // and flip
            Operation::operate(&Flip, tile);
        }
    }
}

impl Operation for Op {
    fn operate(&self, tile: &mut Tile) {
        if let Some(ref rotate) = self.rotate {
            rotate.operate(tile);
        }
        if let Some(ref flip) = self.flip {
            flip.operate(tile);
        }
    }
}
