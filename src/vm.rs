#[derive(Clone, Debug)]
pub enum Opcode {
    Add,
    Mul,
    Sub,
    Div,
    Mov,
    Load,
    Store,
    End,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: [u64; 3],
}
