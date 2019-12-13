#[derive(Debug, Clone)]
pub enum Opcode {
    Add(bool, bool),
    Multiply(bool, bool),
    JumpIfTrue(bool, bool),
    JumpIfFalse(bool, bool),
    LessThan(bool, bool),
    Equals(bool, bool),
    Input,
    Output(bool),
    Halt,
}

pub fn from_num(num: i32) -> Opcode {
    let aux = num.div_euclid(100);
    let m0 = aux.rem_euclid(10) == 1;
    let m1 = aux.div_euclid(10) == 1;
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
