use std::io::{stdin, Read};

fn run(prog: &mut [usize], a: usize, b: usize) -> usize {
    prog[1] = a;
    prog[2] = b;
    let mut pos = 0;
    loop {
        match prog[pos] {
            op @ 1 | op @ 2 => {
                let dst = prog[pos + 3];
                prog[dst] = match op {
                    1 => prog[prog[pos + 1]] + prog[prog[pos + 2]],
                    2 => prog[prog[pos + 1]] * prog[prog[pos + 2]],
                    _ => unreachable!(),
                };
                pos += 4;
            }
            99 => break prog[0],
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut prog = String::new();
    stdin().read_to_string(&mut prog).unwrap();
    let prog = prog
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut prog_buf = prog.clone();
    for a in 0..100 {
        for b in 0..100 {
            prog_buf.copy_from_slice(&prog);
            let result = run(&mut prog_buf, a, b);
            if a == 12 && b == 2 {
                println!("Part 1: {}", result);
            }
            if result == 19690720 {
                println!("Part 2: {}", 100 * a + b);
            }
        }
    }
}
