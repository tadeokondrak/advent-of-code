use std::io::stdin;

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
}

impl Intcode {
    fn new(program: Vec<i32>) -> Self {
        Self {
            memory: program,
            iptr: 0,
        }
    }

    fn load(&self, mode: i32, idx: i32) -> i32 {
        match mode {
            LOAD_POS => self.memory[self.memory[self.iptr as usize + idx as usize] as usize],
            LOAD_IMM => self.memory[self.iptr as usize + idx as usize],
            _ => panic!("invalid load mode: {}", mode),
        }
    }

    fn store(&mut self, idx: i32, val: i32) {
        self.memory[idx as usize] = val
    }

    fn address(&mut self, idx: i32) -> &mut i32 {
        &mut self.memory[idx as usize]
    }

    fn step(&mut self) -> Option<Step> {
        let instr = self.load(LOAD_IMM, 0);
        let mode = (instr / 100 % 10, instr / 1000 % 10);
        let op = instr % 100;
        match op {
            OP_ADD | OP_MUL | OP_LT | OP_EQ => {
                let a = self.load(mode.0, 1);
                let b = self.load(mode.1, 2);
                let dst = self.load(LOAD_IMM, 3);
                match op {
                    OP_ADD => self.store(dst, a + b),
                    OP_MUL => self.store(dst, a * b),
                    OP_LT => self.store(dst, (a < b) as i32),
                    OP_EQ => self.store(dst, (a == b) as i32),
                    _ => unreachable!(),
                };
                self.iptr += 4;
                Some(Step::Continue)
            }
            OP_INPUT => {
                let dst = self.load(LOAD_IMM, 1);
                self.iptr += 2;
                Some(Step::Input(self.address(dst)))
            }
            OP_OUTPUT => {
                let output = self.load(mode.0, 1);
                self.iptr += 2;
                Some(Step::Output(output))
            }
            OP_JMPT | OP_JMPF => {
                let flag = self.load(mode.0, 1);
                let dst = self.load(mode.1, 2);
                if match op {
                    OP_JMPT => flag != 0,
                    OP_JMPF => flag == 0,
                    _ => unreachable!(),
                } {
                    self.iptr = dst;
                } else {
                    self.iptr += 3;
                }
                Some(Step::Continue)
            }
            OP_HALT => {
                self.iptr += 1;
                None
            }
            _ => panic!("invalid opcode: {}", op),
        }
    }

    fn run(&mut self, input: i32) -> i32 {
        let mut output = 0;
        while let Some(step) = self.step() {
            match step {
                Step::Continue => continue,
                Step::Input(i) => *i = input,
                Step::Output(o) => output = o,
            }
        }
        output
    }
}

fn main() {
    let mut prog = String::new();
    stdin().read_line(&mut prog).unwrap();
    let prog: Vec<i32> = prog.trim().split(',').map(|x| x.parse().unwrap()).collect();
    println!(
        "Part 1: {}\nPart 2: {}",
        Intcode::new(prog.clone()).run(1),
        Intcode::new(prog).run(5)
    );
}
