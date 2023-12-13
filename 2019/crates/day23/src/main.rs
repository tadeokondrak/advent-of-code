use std::collections::VecDeque;
use std::io::stdin;

use intcode::{Intcode, Step};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut machines = Vec::new();
    let mut queues = Vec::new();
    for address in 0..50 {
        let mut intcode = Intcode::new(input.trim()).unwrap();
        intcode.run_initial(address, |_| unreachable!()).unwrap();
        machines.push(intcode);
        queues.push(VecDeque::new());
    }
    let mut p1_done = false;
    let mut last_y = None;
    let mut nat = (0, 0);
    loop {
        let mut dirty = false;
        'outer: for i in 0..50 {
            loop {
                match machines[i].step().unwrap() {
                    Step::Continue => continue,
                    Step::Input(inp) => match queues[i].pop_back() {
                        Some(x) => {
                            *inp = x;
                            let y = queues[i].pop_back().unwrap();
                            machines[i].run_initial(y, |_| unreachable!()).unwrap();
                            dirty = true;
                        }
                        None => {
                            *inp = -1;
                            continue 'outer;
                        }
                    },
                    Step::Output(dst) => {
                        let x = machines[i].run_once(|| unreachable!()).unwrap().unwrap();
                        let y = machines[i].run_once(|| unreachable!()).unwrap().unwrap();
                        if dst == 255 {
                            if !p1_done {
                                println!("Part 1: {}", y);
                                p1_done = true;
                            }
                            nat = (x, y);
                        } else {
                            queues[dst as usize].push_front(x);
                            queues[dst as usize].push_front(y);
                        }
                    }
                    Step::End => unreachable!(),
                }
            }
        }
        if !dirty {
            if last_y == Some(nat.1) {
                println!("Part 2: {}", nat.1);
                break;
            }
            queues[0].push_front(nat.0);
            queues[0].push_front(nat.1);
            last_y = Some(nat.1);
        }
    }
}
