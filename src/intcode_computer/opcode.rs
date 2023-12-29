pub enum OpCode {
    Add,
    Mul,
    In,
    Out,
    Halt,
}

impl From<u32> for OpCode {
    fn from(code: u32) -> Self {
        match code {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::In,
            4 => Self::Out,
            99 => Self::Halt,
            _ => unreachable!(),
        }
    }
}
