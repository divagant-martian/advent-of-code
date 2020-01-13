use crate::program::Int;

#[derive(Debug, Clone)]
pub enum Mode {
    Inmediate,
    Position,
    Relative,
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add(Mode, Mode, Mode),
    Multiply(Mode, Mode, Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    SetRelBase(Mode),
    Input(Mode),
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
    let aux2 = aux.div_euclid(10);
    let m1 = Mode::from_num(aux2.rem_euclid(10));
    let m2 = Mode::from_num(aux2.div_euclid(10));
    match num.rem_euclid(100) {
        1 => Opcode::Add(m0, m1, m2),
        2 => Opcode::Multiply(m0, m1, m2),
        3 => Opcode::Input(m0),
        4 => Opcode::Output(m0),
        5 => Opcode::JumpIfTrue(m0, m1),
        6 => Opcode::JumpIfFalse(m0, m1),
        7 => Opcode::LessThan(m0, m1, m2),
        8 => Opcode::Equals(m0, m1, m2),
        9 => Opcode::SetRelBase(m0),
        99 => Opcode::Halt,
        _ => panic!("bad code from num {}", num),
    }
}
