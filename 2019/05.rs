use std::io::stdin;
use std::ops::{Index, IndexMut};

const OP_ADD: i32 = 1;
const OP_MUL: i32 = 2;
const OP_INPUT: i32 = 3;
const OP_OUTPUT: i32 = 4;
const OP_JMPT: i32 = 5;
const OP_JMPF: i32 = 6;
const OP_LT: i32 = 7;
const OP_EQ: i32 = 8;
const OP_HALT: i32 = 99;

const LOAD_POS: i32 = 0;
const LOAD_IMM: i32 = 1;

struct Intcode {
    memory: Vec<i32>,
    iptr: i32,
}

enum Step<'a> {
    Continue,
    Input(&'a mut i32),
    Output(i32),
    Done,
}

impl Intcode {
    fn new(program: Vec<i32>) -> Self {
        Self {
            memory: program,
            iptr: 0,
        }
    }

    fn load(&mut self, mode: i32, idx: i32) -> i32 {
        match mode {
            LOAD_POS => self[self[self.iptr + idx]],
            LOAD_IMM => self[self.iptr + idx],
            _ => panic!("invalid load mode: {}", mode),
        }
    }

    fn step(&mut self) -> Step {
        let instr = self.load(LOAD_IMM, 0);
        let mode2 = instr / 1000 % 10;
        let mode1 = instr / 100 % 10;
        let op = (instr / 10 % 10 * 10) + (instr % 10);
        match op {
            OP_ADD | OP_MUL => {
                let a = self.load(mode1, 1);
                let b = self.load(mode2, 2);
                let dst = self.load(LOAD_IMM, 3);
                self[dst] = match op {
                    OP_ADD => a + b,
                    OP_MUL => a * b,
                    _ => unreachable!(),
                };
                self.iptr += 4;
                Step::Continue
            }
            OP_INPUT => {
                let dst = self.load(LOAD_IMM, 1);
                self.iptr += 2;
                Step::Input(&mut self[dst])
            }
            OP_OUTPUT => {
                let output = self.load(mode1, 1);
                self.iptr += 2;
                Step::Output(output)
            }
            OP_JMPT | OP_JMPF => {
                let flag = self.load(mode1, 1);
                let dst = self.load(mode2, 2);
                if match op {
                    OP_JMPT => flag != 0,
                    OP_JMPF => flag == 0,
                    _ => unreachable!(),
                } {
                    self.iptr = dst;
                } else {
                    self.iptr += 3;
                }
                Step::Continue
            }
            OP_LT | OP_EQ => {
                let a = self.load(mode1, 1);
                let b = self.load(mode2, 2);
                let dst = self.load(LOAD_IMM, 3);
                if match op {
                    OP_LT => a < b,
                    OP_EQ => a == b,
                    _ => unreachable!(),
                } {
                    self[dst] = 1;
                } else {
                    self[dst] = 0;
                }
                self.iptr += 4;
                Step::Continue
            }
            OP_HALT => {
                self.iptr += 1;
                Step::Done
            }
            _ => panic!("invalid opcode: {}", op),
        }
    }

    fn run(&mut self, input: i32) -> i32 {
        let mut output = 0;
        loop {
            match self.step() {
                Step::Continue => continue,
                Step::Input(i) => *i = input,
                Step::Output(o) => output = o,
                Step::Done => break,
            }
        }
        output
    }
}

impl Index<i32> for Intcode {
    type Output = i32;
    fn index(&self, idx: i32) -> &Self::Output {
        &self.memory[idx as usize]
    }
}

impl IndexMut<i32> for Intcode {
    fn index_mut(&mut self, idx: i32) -> &mut Self::Output {
        &mut self.memory[idx as usize]
    }
}

fn main() {
    let mut prog = String::new();
    stdin().read_line(&mut prog).unwrap();

    let prog = prog
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}\nPart 2: {}",
        Intcode::new(prog.clone()).run(1),
        Intcode::new(prog).run(5)
    );
}
