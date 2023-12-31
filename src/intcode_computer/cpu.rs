use super::opcode::OpCode;
use super::opcode::OpCode::*;
use std::{collections::HashMap, io::stdin};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;

#[derive(Clone)]
enum Dat {
    Position(usize),
    Literal(i64),
    Relative(isize),
}

impl From<(u8, i64)> for Dat {
    fn from((mode, value): (u8, i64)) -> Self {
        match mode {
            0 => Self::Position(value as usize),
            1 => Self::Literal(value),
            2 => Self::Relative(value as isize),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Msg {
    Value(i64),
    RxRequest,
}

impl Dat {
    fn value(&self, program: &HashMap<usize, i64>, base: &isize) -> i64 {
        match self {
            Dat::Position(v) => *program.get(v).unwrap_or(&0),
            Dat::Literal(v) => *v,
            Dat::Relative(v) => *program.get(&((base + v) as usize)).unwrap_or(&0),
        }
    }

    fn addr(&self, base: &isize) -> usize {
        match self {
            Dat::Position(v) => *v,
            Dat::Relative(v) => (base + v) as usize,
            _ => panic!("tried to get addr of a non positional or relative argument"),
        }
    }
}

impl Dat {}

pub struct Cpu {
    pub program: HashMap<usize, i64>,
    pc: usize,
    pub relative_base: isize,
    pub outputs: Vec<i64>,
    input: Option<Receiver<Msg>>,
}

impl Cpu {
    pub fn new(program: Vec<i64>) -> Self {
        let program: HashMap<usize, i64> = program.into_iter().enumerate().collect();
        Self {
            program,
            pc: 0,
            relative_base: 0,
            outputs: Vec::new(),
            input: None,
        }
    }

    pub fn new_async(program: Vec<i64>) -> (Self, Sender<Msg>) {
        let (tx_handle, rx) = mpsc::channel(32);
        let mut instance = Self::new(program);
        instance.input = Some(rx);
        (instance, tx_handle)
    }

    fn get(&self, arg: &Dat) -> i64 {
        arg.value(&self.program, &self.relative_base)
    }

    fn get_mut(&mut self, arg: &Dat) -> &mut i64 {
        self.program
            .entry(arg.addr(&self.relative_base))
            .or_default()
    }

    fn run_common(&mut self) -> (OpCode, Dat) {
        let (opcode, a, b, c) = self.advance();

        match opcode {
            Add => *self.get_mut(&c) = self.get(&a) + self.get(&b),
            Mul => *self.get_mut(&c) = self.get(&a) * self.get(&b),
            Jt => {
                if self.get(&a) != 0 {
                    self.pc = self.get(&b) as usize
                }
            }
            Jf => {
                if self.get(&a) == 0 {
                    self.pc = self.get(&b) as usize
                }
            }
            Lt => *self.get_mut(&c) = (self.get(&a) < self.get(&b)) as i64,
            Eq => *self.get_mut(&c) = (self.get(&a) == self.get(&b)) as i64,
            Rb => self.relative_base += self.get(&a) as isize,
            Halt | In | Out => (),
        }
        (opcode, a)
    }

    pub fn run(&mut self, inputs: Option<Vec<i64>>) {
        let mut inputs = inputs.map(|v| v.into_iter());

        loop {
            let (opcode, a) = self.run_common();
            match opcode {
                In => *self.get_mut(&a) = self.input(&mut inputs),
                Out => self.output(self.get(&a)),
                Halt => break,
                _ => (),
            }
        }
    }

    pub async fn run_async(&mut self, tx: Sender<Msg>) {
        loop {
            let (opcode, a) = self.run_common();
            match opcode {
                In => *self.get_mut(&a) = self.input_async(&tx).await,
                Out => self.output_async(self.get(&a), &tx).await,
                Halt => break,
                _ => (),
            }
        }
    }

    fn advance(&mut self) -> (OpCode, Dat, Dat, Dat) {
        let opcode = *self.program.entry(self.pc).or_default();
        let a = *self.program.entry(self.pc + 1).or_default();
        let b = *self.program.entry(self.pc + 2).or_default();
        let c = *self.program.entry(self.pc + 3).or_default();

        let a = Dat::from((((opcode / 100) % 10) as u8, a));
        let b = Dat::from((((opcode / 1_000) % 10) as u8, b));
        let c = Dat::from((((opcode / 10_000) % 10) as u8, c));

        let opcode = OpCode::from(opcode);

        match opcode {
            In | Out | Rb => self.pc += 2,
            Jt | Jf => self.pc += 3,
            Add | Mul | Lt | Eq => self.pc += 4,
            Halt => (),
        }

        (opcode, a, b, c)
    }

    fn input(&mut self, inputs: &mut Option<impl Iterator<Item = i64>>) -> i64 {
        if let Some(inputs) = inputs {
            inputs.next().unwrap()
        } else {
            let mut input_line = String::new();
            stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");

            input_line
                .trim()
                .parse::<i64>()
                .expect("Could not parse integer from input")
        }
    }

    async fn input_async(&mut self, tx: &Sender<Msg>) -> i64 {
        tx.send(Msg::RxRequest).await.unwrap();
        loop {
            if let Msg::Value(value) = self.input.as_mut().unwrap().recv().await.unwrap() {
                return value;
            }
        }
    }

    fn output(&mut self, value: i64) {
        self.outputs.push(value);
        info!("{}", value)
    }

    async fn output_async(&mut self, value: i64, tx: &Sender<Msg>) {
        tx.send(Msg::Value(value)).await.unwrap();
        self.output(value);
    }
}
