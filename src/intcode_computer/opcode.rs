pub enum OpCode {
    Add,
    Mul,
    In,
    Out,
    Jt,
    Jf,
    Lt,
    Eq,
    Rb,
    Halt,
}

impl From<i64> for OpCode {
    fn from(code: i64) -> Self {
        match code % 100 {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::In,
            4 => Self::Out,
            5 => Self::Jt,
            6 => Self::Jf,
            7 => Self::Lt,
            8 => Self::Eq,
            9 => Self::Rb,
            99 => Self::Halt,
            _ => unreachable!(),
        }
    }
}
