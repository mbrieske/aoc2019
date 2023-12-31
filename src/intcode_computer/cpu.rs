use super::opcode::OpCode;
use super::opcode::OpCode::*;
use std::io::stdin;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;

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

    fn get(&self, arg: Dat) -> i32 {
        arg.value(&self.program)
    }

    fn get_mut(&mut self, arg: Dat) -> &mut i32 {
        &mut self.program[arg.addr()]
    }

    pub async fn run_async(&mut self, inputs: Option<Vec<i32>>) {
        let mut inputs = inputs.map(|v| v.into_iter());
        loop {
            let (opcode, a, b, c) = self.advance();

            match opcode {
                Add => *self.get_mut(c) = self.get(a) + self.get(b),
                Mul => *self.get_mut(c) = self.get(a) * self.get(b),
                In => *self.get_mut(a) = self.get_input(&mut inputs).await,
                Out => self.output(self.get(a)).await,
                Jt => {
                    if self.get(a) != 0 {
                        self.pc = self.get(b) as usize
                    }
                }
                Jf => {
                    if self.get(a) == 0 {
                        self.pc = self.get(b) as usize
                    }
                }
                Lt => *self.get_mut(c) = (self.get(a) < self.get(b)) as i32,
                Eq => *self.get_mut(c) = (self.get(a) == self.get(b)) as i32,
                Halt => break,
            }
        }
    }

    pub fn run(&mut self, inputs: Option<Vec<i32>>) {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.run_async(inputs))
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

    async fn get_input(&mut self, inputs: &mut Option<impl Iterator<Item = i32>>) -> i32 {
        if self.r#async {
            self.input.as_mut().unwrap().recv().await.unwrap()
        } else if let Some(input) = inputs.as_mut() {
            input.next().unwrap()
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

    async fn output(&mut self, value: i32) {
        if self.r#async {
            self.output.as_ref().unwrap().send(value).await.unwrap();
        }
        self.outputs.push(value);
        info!("{}", value)
    }
}
