use std::io::stdin;

fn main() {
    let mut prog = String::new();
    stdin().read_line(&mut prog).unwrap();
    let prog: Vec<usize> = prog.trim().split(',').map(|x| x.parse().unwrap()).collect();

    for a in 0..100 {
        for b in 0..100 {
            let mut prog = prog.clone();
            prog[1] = a;
            prog[2] = b;
            let result = (0..)
                .step_by(4)
                .find_map(|pos| match prog[pos] {
                    op @ 1 | op @ 2 => {
                        let dst = prog[pos + 3];
                        prog[dst] = match op {
                            1 => prog[prog[pos + 1]] + prog[prog[pos + 2]],
                            2 => prog[prog[pos + 1]] * prog[prog[pos + 2]],
                            _ => unreachable!(),
                        };
                        None
                    }
                    _ => Some(prog[0]),
                })
                .unwrap();
            if a == 12 && b == 2 {
                println!("Part 1: {}", result);
            }
            if result == 19690720 {
                println!("Part 2: {}", 100 * a + b);
            }
        }
    }
}
