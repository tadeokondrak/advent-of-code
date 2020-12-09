use std::io::{stdin, BufRead};

fn main() {
    let nums = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<i64>>();

    let p1 = (0..nums.len() - 25)
        .map(|i| (&nums[i..i + 25], nums[i + 25]))
        .filter(|&(preamble, num)| {
            !preamble
                .iter()
                .flat_map(|x| preamble.iter().map(move |y| (x, y)))
                .filter(|(x, y)| x != y)
                .any(|(x, y)| x + y == num)
        })
        .next()
        .unwrap()
        .1;

    let p2 = (2..nums.len())
        .flat_map(|n| nums.windows(n))
        .filter(|window| window.iter().sum::<i64>() == p1)
        .map(|window| {
            let mut window = Vec::from(window);
            window.sort();
            window
        })
        .map(|window| window[0] + window[window.len() - 1])
        .next()
        .unwrap();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
