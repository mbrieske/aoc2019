pub enum OpCode {
    Add,
    Mul,
    In,
    Out,
    Halt,
}

impl From<i32> for OpCode {
    fn from(code: i32) -> Self {
        match code % 100 {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::In,
            4 => Self::Out,
            99 => Self::Halt,
            _ => unreachable!(),
        }
    }
}
