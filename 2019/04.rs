use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input
        .trim()
        .split('-')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<usize>>();

    let iter = (input[0]..=input[1])
        .map(|n| n.to_string().bytes().map(|x| x - b'0').collect::<Vec<u8>>())
        .filter(|n| !n.windows(2).any(|x| x[0] > x[1]));

    let p1 = iter
        .clone()
        .filter(|n| n.windows(2).any(|x| x[0] == x[1]))
        .count();

    let p2 = iter
        .filter(|n| {
            n[0] == n[1] && n[1] != n[2]
                || n.windows(4)
                    .any(|x| x[0] != x[1] && x[1] == x[2] && x[2] != x[3])
                || n[4] == n[5] && n[3] != n[4]
        })
        .count();

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
