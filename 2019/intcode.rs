#![allow(dead_code)]

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
enum Step<'a> {
    Continue,
    Input(&'a mut i64),
    Output(i64),
    End,
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    InvalidOpcode(i64),
    InvalidParameterMode(i64),
    OutOfBoundsLoad(i64),
    OutOfBoundsStore(i64),
    #[cfg(debug_assertions)]
    AddOverflow(i64, i64),
    #[cfg(debug_assertions)]
    MulOverflow(i64, i64),
    NotEnoughInputs,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidOpcode(op) => write!(f, "Invalid opcode {}", op),
            Error::InvalidParameterMode(mode) => write!(f, "Invalid parameter mode {}", mode),
            Error::OutOfBoundsLoad(idx) => write!(f, "Out of bounds load to index {}", idx),
            Error::OutOfBoundsStore(idx) => write!(f, "Out of bounds store to index {}", idx),
            #[cfg(debug_assertions)]
            Error::AddOverflow(a, b) => write!(f, "Addition overflow: {} + {}", a, b),
            #[cfg(debug_assertions)]
            Error::MulOverflow(a, b) => write!(f, "Multiplication overflow: {} * {}", a, b),
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

    fn get(&mut self, idx: usize) -> &mut i64 {
        if self.mem.len() < idx as usize + 1 {
            self.mem.resize(idx as usize + 1, 0);
        }
        &mut self.mem[idx]
    }

    fn load(&mut self, mode: i64, idx: i64) -> Result<i64, Error> {
        match mode {
            LOAD_POS => {
                let idx = *self.get(self.ip as usize + idx as usize);
                Ok(*self.get(idx as usize))
            }
            LOAD_IMM => Ok(*self.get(self.ip as usize + idx as usize)),
            LOAD_REL => {
                let idx = *self.get(self.ip as usize + idx as usize);
                Ok(*self.get((idx as isize + self.base as isize) as usize))
            }
            _ => Err(Error::InvalidParameterMode(mode)),
        }
    }

    fn store(&mut self, mode: i64, idx: i64, val: i64) -> Result<(), Error> {
        *self.address(mode, idx)? = val;
        Ok(())
    }

    fn address(&mut self, mode: i64, idx: i64) -> Result<&mut i64, Error> {
        let idx = match mode {
            LOAD_POS => *self.get((self.ip as isize + idx as isize) as usize),
            LOAD_REL => *self.get((self.ip as isize + idx as isize) as usize) + self.base,
            _ => return Err(Error::InvalidParameterMode(mode)),
        };
        Ok(self.get(idx as usize))
    }

    fn step(&mut self) -> Result<Step, Error> {
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
                        #[cfg(debug_assertions)]
                        OP_ADD => a.checked_add(b).ok_or(Error::AddOverflow(a, b))?,
                        #[cfg(not(debug_assertions))]
                        OP_ADD => a.wrapping_add(b),
                        #[cfg(debug_assertions)]
                        OP_MUL => a.checked_mul(b).ok_or(Error::MulOverflow(a, b))?,
                        #[cfg(not(debug_assertions))]
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
                let flag = self.load(mode.0, 1)?;
                let dest = self.load(mode.1, 2)?;
                if match op {
                    OP_JNZ => flag != 0,
                    OP_JZ => flag == 0,
                    _ => unreachable!(),
                } {
                    self.ip = dest;
                } else {
                    self.ip += 3;
                }
                Ok(Step::Continue)
            }
            OP_RELBASE => {
                let diff = self.load(mode.0, 1)?;
                self.base += diff;
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
