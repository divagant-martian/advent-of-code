use crate::program::Int;

#[derive(Debug, Clone)]
pub enum Mode {
    Inmediate,
    Position,
    Relative,
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
    Input,
    Output(Mode),
    Halt,
}

impl Mode {
    pub fn from_num(num: Int) -> Self {
        match num {
            0 => Mode::Position,
            1 => Mode::Inmediate,
            2 => Mode::Relative,
            _ => panic!("bad mode from num {}", num),
        }
    }
}
pub fn from_num(num: Int) -> Opcode {
    let aux = num.div_euclid(100);
    let m0 = Mode::from_num(aux.rem_euclid(10));
    let m1 = Mode::from_num(aux.div_euclid(10));
    match num.rem_euclid(100) {
        1 => Opcode::Add(m0, m1),
        2 => Opcode::Multiply(m0, m1),
        3 => Opcode::Input,
        4 => Opcode::Output(m0),
        5 => Opcode::JumpIfTrue(m0, m1),
        6 => Opcode::JumpIfFalse(m0, m1),
        7 => Opcode::LessThan(m0, m1),
        8 => Opcode::Equals(m0, m1),
        99 => Opcode::Halt,
        _ => panic!("bad code from num {}", num),
    }
}
