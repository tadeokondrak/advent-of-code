use std::fmt::{self, Display};
use std::io::stdin;

const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_IN: i32 = 3;
const OP_OUT: i32 = 4;
const OP_JNZ: i32 = 5;
const OP_JZ: i32 = 6;
const OP_LT: i32 = 7;
const OP_EQ: i32 = 8;
const OP_HLT: i32 = 99;

const LOAD_POS: i32 = 0;
const LOAD_IMM: i32 = 1;

struct Intcode {
    memory: Vec<i32>,
    ip: i32,
}

enum Step<'a> {
    Continue,
    Input(&'a mut i32),
    Output(i32),
    End,
}

#[derive(Debug)]
enum Error {
    InvalidOpcode(i32),
    InvalidParameterMode(i32),
    OutOfBoundsLoad(i32),
    OutOfBoundsStore(i32),
    NotEnoughInputs,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidOpcode(op) => write!(f, "Invalid opcode {}", op),
            Error::InvalidParameterMode(mode) => write!(f, "Invalid parameter mode {}", mode),
            Error::OutOfBoundsLoad(idx) => write!(f, "Out of bounds load to index {}", idx),
            Error::OutOfBoundsStore(idx) => write!(f, "Out of bounds store to index {}", idx),
            Error::NotEnoughInputs => write!(f, "Not enough inputs"),
        }
    }
}

impl Intcode {
    fn new(program: Vec<i32>) -> Self {
        Self {
            memory: program,
            ip: 0,
        }
    }

    fn load(&self, mode: i32, idx: i32) -> Result<i32, Error> {
        match mode {
            LOAD_POS => match self.memory.get(self.ip as usize + idx as usize) {
                Some(&idx) => match self.memory.get(idx as usize) {
                    Some(&val) => Ok(val),
                    None => Err(Error::OutOfBoundsLoad(idx)),
                },
                None => Err(Error::OutOfBoundsLoad(self.ip + idx)),
            },
            LOAD_IMM => match self.memory.get(self.ip as usize + idx as usize) {
                Some(&val) => Ok(val),
                None => Err(Error::OutOfBoundsLoad(self.ip + idx)),
            },
            _ => Err(Error::InvalidParameterMode(mode)),
        }
    }

    fn store(&mut self, idx: i32, val: i32) -> Result<(), Error> {
        match self.memory.get_mut(idx as usize) {
            Some(m) => Ok(*m = val),
            None => Err(Error::OutOfBoundsStore(idx)),
        }
    }

    fn address(&mut self, idx: i32) -> Result<&mut i32, Error> {
        match self.memory.get_mut(idx as usize) {
            Some(m) => Ok(m),
            None => Err(Error::OutOfBoundsStore(idx)),
        }
    }

    fn step(&mut self) -> Result<Step, Error> {
        let instr = self.load(LOAD_IMM, 0)?;
        let mode = (instr / 100 % 10, instr / 1000 % 10);
        let op = instr % 100;
        match op {
            OP_ADD | OP_MUL | OP_LT | OP_EQ => {
                let a = self.load(mode.0, 1)?;
                let b = self.load(mode.1, 2)?;
                let dst = self.load(LOAD_IMM, 3)?;
                self.store(
                    dst,
                    match op {
                        OP_ADD => a + b,
                        OP_MUL => a * b,
                        OP_LT => (a < b) as i32,
                        OP_EQ => (a == b) as i32,
                        _ => unreachable!(),
                    },
                )?;
                self.ip += 4;
                Ok(Step::Continue)
            }
            OP_IN => {
                let dst = self.load(LOAD_IMM, 1)?;
                self.ip += 2;
                Ok(Step::Input(self.address(dst)?))
            }
            OP_OUT => {
                let output = self.load(mode.0, 1)?;
                self.ip += 2;
                Ok(Step::Output(output))
            }
            OP_JNZ | OP_JZ => {
                let flag = self.load(mode.0, 1)?;
                let dst = self.load(mode.1, 2)?;
                if match op {
                    OP_JNZ => flag != 0,
                    OP_JZ => flag == 0,
                    _ => unreachable!(),
                } {
                    self.ip = dst;
                } else {
                    self.ip += 3;
                }
                Ok(Step::Continue)
            }
            OP_HLT => {
                self.ip += 1;
                Ok(Step::End)
            }
            _ => Err(Error::InvalidOpcode(op)),
        }
    }

    fn run(&mut self, input: &[i32]) -> Result<Vec<i32>, Error> {
        let mut inputs = input.iter();
        let mut outputs = Vec::new();
        loop {
            match self.step()? {
                Step::Continue => continue,
                Step::Input(i) => {
                    *i = *match inputs.next() {
                        Some(i) => i,
                        None => return Err(Error::NotEnoughInputs),
                    }
                }
                Step::Output(o) => outputs.push(o),
                Step::End => return Ok(outputs),
            }
        }
    }
}

fn main() {
    let mut prog = String::new();
    stdin().read_line(&mut prog).unwrap();
    let prog: Vec<i32> = prog.trim().split(',').map(|x| x.parse().unwrap()).collect();
    println!(
        "Part 1: {}\nPart 2: {}",
        Intcode::new(prog.clone()).run(&[1]).unwrap()[9],
        Intcode::new(prog).run(&[5]).unwrap()[0]
    );
}
