#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Velocity(i32, i32, i32);
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Position(i32, i32, i32);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Moon {
    velocity: Velocity,
    pub position: Position,
}

impl Moon {
    pub fn new(xv: i32, yv: i32, zv: i32) -> Self {
        Moon {
            position: Position(xv, yv, zv),
            velocity: Velocity(0, 0, 0),
        }
    }

    pub fn kin_energy(&self) -> i32 {
        let Velocity(x, y, z) = self.velocity;
        x.abs() + y.abs() + z.abs()
    }

    pub fn pot_energy(&self) -> i32 {
        let Position(x, y, z) = self.position;
        x.abs() + y.abs() + z.abs()
    }

    pub fn total_energy(&self) -> i32 {
        self.pot_energy() * self.kin_energy()
    }

    pub fn cmp(&self, other: &Self) -> (i32, i32, i32) {
        let Position(x0, y0, z0) = self.position;
        let Position(x1, y1, z1) = other.position;
        ((x0 - x1).signum(), (y0 - y1).signum(), (z0 - z1).signum())
    }
    pub fn add_velocity(&mut self, signs: (i32, i32, i32), scalar: i32) {
        self.velocity.0 += scalar * signs.0;
        self.velocity.1 += scalar * signs.1;
        self.velocity.2 += scalar * signs.2;
    }

    pub fn calc_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }
}

pub fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let cmp = moons[i].cmp(&moons[j]);
            moons[i].add_velocity(cmp, -1);
            moons[j].add_velocity(cmp, 1);
        }
    }
    for i in 0..moons.len() {
        moons[i].calc_velocity();
    }
}

pub fn sim(moons: &mut [Moon], steps: i32) {
    for _ in 1..=steps {
        step(moons);
    }
}
