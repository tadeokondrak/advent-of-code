use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Debug, Copy, Clone)]
enum Ins {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

fn run_program(program: &[Ins]) -> Result<i32, i32> {
    let (mut ip, mut acc) = (0, 0);
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(ip) {
            break Err(acc);
        }
        if ip as usize == program.len() {
            break Ok(acc);
        }
        match program[ip as usize] {
            Ins::Jmp(arg) => {
                ip += arg;
            }
            Ins::Acc(arg) => {
                acc += arg;
                ip += 1;
            }
            Ins::Nop(_) => {
                ip += 1;
            }
        }
    }
}

fn main() {
    let mut program = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut it = line.split(' ');
            (match it.next().unwrap() {
                "jmp" => Ins::Jmp,
                "acc" => Ins::Acc,
                "nop" => Ins::Nop,
                _ => unreachable!(),
            })(it.next().unwrap().parse().unwrap())
        })
        .collect::<Vec<Ins>>();
    println!("Part 1: {}", run_program(&program).unwrap_err());
    for idx in 0..program.len() {
        match program[idx] {
            Ins::Jmp(arg) => {
                program[idx] = Ins::Nop(arg);
                if let Ok(acc) = run_program(&program) {
                    println!("Part 2: {}", acc);
                    break;
                }
                program[idx] = Ins::Jmp(arg);
            }
            Ins::Acc(_) => continue,
            Ins::Nop(arg) => {
                program[idx] = Ins::Jmp(arg);
                if let Ok(acc) = run_program(&program) {
                    println!("Part 2: {}", acc);
                    break;
                }
                program[idx] = Ins::Nop(arg);
            }
        }
    }
}
