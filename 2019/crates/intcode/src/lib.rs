use std::fmt::{self, Display};
use std::num::ParseIntError;

const OP_ADD: i64 = 1;
const OP_MUL: i64 = 2;
const OP_IN: i64 = 3;
const OP_OUT: i64 = 4;
const OP_JNZ: i64 = 5;
const OP_JZ: i64 = 6;
const OP_LT: i64 = 7;
const OP_EQ: i64 = 8;
const OP_RELBASE: i64 = 9;
const OP_HLT: i64 = 99;

const LOAD_POS: i64 = 0;
const LOAD_IMM: i64 = 1;
const LOAD_REL: i64 = 2;

#[derive(Debug, Clone)]
pub struct Intcode {
    pub mem: Vec<i64>,
    pub ip: i64,
    pub base: i64,
}

#[derive(Debug)]
pub enum Step<'a> {
    Continue,
    Input(&'a mut i64),
    Output(i64),
    End,
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    InvalidOpcode(i64),
    InvalidParameterMode(i64),
    NotEnoughInputs,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidOpcode(op) => write!(f, "Invalid opcode {}", op),
            Error::InvalidParameterMode(mode) => write!(f, "Invalid parameter mode {}", mode),
            Error::NotEnoughInputs => write!(f, "Not enough inputs"),
        }
    }
}

impl Intcode {
    pub fn new(program: &str) -> Result<Self, ParseIntError> {
        let mut mem = Vec::new();
        let split = program.split(',');
        for instruction in split {
            mem.push(instruction.parse()?);
        }
        Ok(Self {
            mem,
            ip: 0,
            base: 0,
        })
    }

    fn load(&mut self, mode: i64, idx: i64) -> Result<i64, Error> {
        match mode {
            LOAD_POS => match self.mem.get(self.ip as usize + idx as usize) {
                Some(&idx) => Ok(*self.mem.get(idx as usize).unwrap_or(&0)),
                None => Ok(0),
            },
            LOAD_IMM => Ok(*self.mem.get(self.ip as usize + idx as usize).unwrap_or(&0)),
            LOAD_REL => match self.mem.get(self.ip as usize + idx as usize) {
                Some(&idx) => Ok(*self
                    .mem
                    .get((idx as isize + self.base as isize) as usize)
                    .unwrap_or(&0)),
                None => Ok(0),
            },
            _ => Err(Error::InvalidParameterMode(mode)),
        }
    }

    fn store(&mut self, mode: i64, idx: i64, val: i64) -> Result<(), Error> {
        Ok(*self.address(mode, idx)? = val)
    }

    fn address(&mut self, mode: i64, idx: i64) -> Result<&mut i64, Error> {
        let idx = match mode {
            LOAD_POS => *self
                .mem
                .get((self.ip as isize + idx as isize) as usize)
                .unwrap_or(&0),
            LOAD_REL => {
                *self
                    .mem
                    .get((self.ip as isize + idx as isize) as usize)
                    .unwrap_or(&0)
                    + self.base
            }
            _ => return Err(Error::InvalidParameterMode(mode)),
        };
        if self.mem.len() <= idx as usize {
            self.mem.resize(idx as usize + 1, 0);
        }
        Ok(&mut self.mem[idx as usize])
    }

    pub fn step(&mut self) -> Result<Step, Error> {
        let instr = self.load(LOAD_IMM, 0)?;
        let mode = (instr / 100 % 10, instr / 1000 % 10, instr / 10000 % 10);
        let op = instr % 100;
        match op {
            OP_ADD | OP_MUL | OP_LT | OP_EQ => {
                let a = self.load(mode.0, 1)?;
                let b = self.load(mode.1, 2)?;
                self.store(
                    mode.2,
                    3,
                    match op {
                        OP_ADD => a.wrapping_add(b),
                        OP_MUL => a.wrapping_mul(b),
                        OP_LT => (a < b) as i64,
                        OP_EQ => (a == b) as i64,
                        _ => unreachable!(),
                    },
                )?;
                self.ip += 4;
                Ok(Step::Continue)
            }
            OP_IN => {
                self.ip += 2;
                Ok(Step::Input(self.address(mode.0, -1)?))
            }
            OP_OUT => {
                let output = self.load(mode.0, 1)?;
                self.ip += 2;
                Ok(Step::Output(output))
            }
            OP_JNZ | OP_JZ => {
                if match (op, self.load(mode.0, 1)?) {
                    (OP_JNZ, 0) => false,
                    (OP_JNZ, _) => true,
                    (OP_JZ, 0) => true,
                    (OP_JZ, _) => false,
                    _ => unreachable!(),
                } {
                    self.ip = self.load(mode.1, 2)?;
                } else {
                    self.ip += 3;
                }
                Ok(Step::Continue)
            }
            OP_RELBASE => {
                self.base += self.load(mode.0, 1)?;
                self.ip += 2;
                Ok(Step::Continue)
            }
            OP_HLT => {
                self.ip += 1;
                Ok(Step::End)
            }
            _ => Err(Error::InvalidOpcode(op)),
        }
    }

    /// Run the program until it halts, using the provided callbacks when it asks for input or
    /// outputs something.
    pub fn run(
        &mut self,
        mut input: impl FnMut() -> i64,
        mut output: impl FnMut(i64),
    ) -> Result<(), Error> {
        loop {
            match self.step()? {
                Step::Continue => {}
                Step::Input(i) => *i = input(),
                Step::Output(o) => output(o),
                Step::End => break Ok(()),
            }
        }
    }

    /// Run the program until it either requests input (and provide `initial_input`) or halts.
    pub fn run_initial(
        &mut self,
        initial_input: i64,
        mut output: impl FnMut(i64),
    ) -> Result<bool, Error> {
        loop {
            match self.step()? {
                Step::Continue => {}
                Step::Input(i) => {
                    *i = initial_input;
                    break Ok(true);
                }
                Step::Output(o) => output(o),
                Step::End => break Ok(false),
            }
        }
    }

    /// Run the program until it outputs something or it halts.
    pub fn run_once(&mut self, mut input: impl FnMut() -> i64) -> Result<Option<i64>, Error> {
        loop {
            match self.step()? {
                Step::Continue => {}
                Step::Input(i) => *i = input(),
                Step::Output(o) => break Ok(Some(o)),
                Step::End => break Ok(None),
            }
        }
    }
}
