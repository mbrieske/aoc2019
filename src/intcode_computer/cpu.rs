use super::opcode::OpCode;
use super::opcode::OpCode::*;
use std::io::stdin;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;

#[derive(Clone)]
enum Dat {
    Reference(usize),
    Literal(i32),
}

impl Dat {
    fn value(&self, program: &[i32]) -> i32 {
        match self {
            Dat::Reference(v) => *program.get(*v).unwrap(),
            Dat::Literal(v) => *v,
        }
    }

    fn addr(&self) -> usize {
        match self {
            Dat::Reference(v) => *v,
            _ => panic!("tried to get addr of a non positional argument"),
        }
    }
}

impl Dat {}

pub struct Cpu {
    pub program: Vec<i32>,
    pc: usize,
    pub outputs: Vec<i32>,
    r#async: bool,
    pub output: Option<Sender<i32>>,
    input: Option<Receiver<i32>>,
}

impl Cpu {
    pub fn new(mut program: Vec<i32>) -> Self {
        program.extend([0; 3]);
        Self {
            program,
            pc: 0,
            outputs: Vec::new(),
            r#async: false,
            input: None,
            output: None,
        }
    }

    pub fn new_async(mut program: Vec<i32>, tx: Option<Sender<i32>>) -> (Self, Sender<i32>) {
        let (tx_handle, rx) = mpsc::channel(32);
        program.extend([0; 3]);
        (
            Self {
                program,
                pc: 0,
                outputs: Vec::new(),
                r#async: true,
                output: tx,
                input: Some(rx),
            },
            tx_handle,
        )
    }

    fn get(&self, arg: &Dat) -> i32 {
        arg.value(&self.program)
    }

    fn get_mut(&mut self, arg: &Dat) -> &mut i32 {
        &mut self.program[arg.addr()]
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
            Lt => *self.get_mut(&c) = (self.get(&a) < self.get(&b)) as i32,
            Eq => *self.get_mut(&c) = (self.get(&a) == self.get(&b)) as i32,
            Halt | In | Out => (),
        }
        (opcode, a)
    }

    pub fn run(&mut self, inputs: Option<Vec<i32>>) {
        let mut inputs = inputs.map(|v| v.into_iter());

        loop {
            let (opcode, a) = self.run_common();
            match opcode {
                In => *self.get_mut(&a) = self.get_input(&mut inputs),
                Out => self.output(self.get(&a)),
                Halt => break,
                _ => (),
            }
        }
    }

    pub async fn run_async(&mut self) {
        loop {
            let (opcode, a) = self.run_common();
            match opcode {
                In => *self.get_mut(&a) = self.get_input_async().await,
                Out => self.output_async(self.get(&a)).await,
                Halt => break,
                _ => (),
            }
        }
    }

    fn advance(&mut self) -> (OpCode, Dat, Dat, Dat) {
        let [opcode, a, b, c] = self.program[self.pc..self.pc + 4] else {
            unreachable!()
        };
        let a = if (opcode / 100) % 10 == 0 {
            Dat::Reference(a as usize)
        } else {
            Dat::Literal(a)
        };
        let b = if (opcode / 1_000) % 10 == 0 {
            Dat::Reference(b as usize)
        } else {
            Dat::Literal(b)
        };
        let c = if (opcode / 10_000) % 10 == 0 {
            Dat::Reference(c as usize)
        } else {
            Dat::Literal(c)
        };
        let opcode = OpCode::from(opcode);

        match opcode {
            In | Out => self.pc += 2,
            Jt | Jf => self.pc += 3,
            Add | Mul | Lt | Eq => self.pc += 4,
            Halt => (),
        }

        (opcode, a, b, c)
    }

    fn get_input(&mut self, inputs: &mut Option<impl Iterator<Item = i32>>) -> i32 {
        if let Some(inputs) = inputs {
            inputs.next().unwrap()
        } else {
            let mut input_line = String::new();
            stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");

            input_line
                .trim()
                .parse::<i32>()
                .expect("Could not parse integer from input")
        }
    }

    async fn get_input_async(&mut self) -> i32 {
        self.input.as_mut().unwrap().recv().await.unwrap()
    }

    fn output(&mut self, value: i32) {
        self.outputs.push(value);
        info!("{}", value)
    }

    async fn output_async(&mut self, value: i32) {
        if self.r#async {
            self.output.as_ref().unwrap().send(value).await.unwrap();
        }
        self.output(value);
    }
}
