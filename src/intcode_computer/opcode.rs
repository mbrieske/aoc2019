pub enum OpCode {
    Add,
    Mul,
    Halt,
}

impl From<u32> for OpCode {
    fn from(code: u32) -> Self {
        match code {
            1 => Self::Add,
            2 => Self::Mul,
            99 => Self::Halt,
            _ => unreachable!(),
        }
    }
}
