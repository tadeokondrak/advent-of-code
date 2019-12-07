use std::cell::Cell;
use std::io::stdin;

mod intcode;
use intcode::Intcode;

fn run(intcode: &Intcode, inputs: &[i32]) -> i32 {
    let out = Cell::new(0);
    let mut intcodes = inputs
        .iter()
        .map(|&n| {
            let mut ic = intcode.clone();
            ic.run_initial(n, |_| unreachable!()).unwrap();
            ic
        })
        .collect::<Vec<_>>();

    let (first, rest) = intcodes.split_at_mut(1);
    first[0]
        .run(
            || out.get(),
            |last| {
                let x = rest
                    .into_iter()
                    .fold(last, |last, ic| ic.run_once(|| last).unwrap().unwrap());
                out.set(x);
            },
        )
        .unwrap();

    out.get()
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let intcode = Intcode::new(input.trim()).unwrap();
    let (mut p1, mut p2) = (0, 0);
    for a in 0..=4 {
        for b in (0..=4).filter(|&b| a != b) {
            for c in (0..=4).filter(|&c| a != c && b != c) {
                for d in (0..=4).filter(|&d| a != d && b != d && c != d) {
                    for e in (0..=4).filter(|&e| a != e && b != e && c != e && d != e) {
                        p1 = p1.max(run(&intcode, &[a, b, c, d, e]));
                        p2 = p2.max(run(&intcode, &[a + 5, b + 5, c + 5, d + 5, e + 5]));
                    }
                }
            }
        }
    }
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
