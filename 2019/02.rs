use std::io::stdin;

mod intcode;
use intcode::Intcode;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let intcode = Intcode::new(input.trim()).unwrap();
    for a in 0..100 {
        for b in 0..100 {
            let mut intcode = intcode.clone();
            intcode.mem[1] = a;
            intcode.mem[2] = b;
            intcode.run(|| unreachable!(), |_| unreachable!()).unwrap();
            let result = intcode.mem[0];
            if a == 12 && b == 2 {
                println!("Part 1: {}", result);
            }
            if result == 19690720 {
                println!("Part 2: {}", 100 * a + b);
            }
        }
    }
}
