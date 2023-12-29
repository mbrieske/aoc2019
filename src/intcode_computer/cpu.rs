use std::io::stdin;

use super::opcode::OpCode;

pub struct Cpu {
    pub program: Vec<u32>,
    pc: usize,
}

impl Cpu {
    pub fn new(mut program: Vec<u32>) -> Self {
        program.extend([255; 3]);
        Self { program, pc: 0 }
    }

    pub fn run(&mut self) {
        use super::opcode::OpCode::*;
        loop {
            let [opcode, a, b, c] = self.program[self.pc..self.pc + 4] else {
                unreachable!()
            };
            let opcode = OpCode::from(opcode);
            let a = a as usize;
            let b = b as usize;
            let c = c as usize;

            match opcode {
                In | Out => self.pc += 2,
                Add | Mul => self.pc += 4,
                Halt => (),
            }

            match opcode {
                OpCode::Add => self.program[c] = self.program[a] + self.program[b],
                OpCode::Mul => self.program[c] = self.program[a] * self.program[b],
                OpCode::In => {
                    let mut input_line = String::new();
                    stdin()
                        .read_line(&mut input_line)
                        .expect("Failed to read line");
                    let input = input_line
                        .trim()
                        .parse()
                        .expect("Could not parse integer from input");
                    self.program[a] = input;
                }
                OpCode::Out => println!("{}", self.program[a]),
                OpCode::Halt => break,
            }
        }
    }
}
