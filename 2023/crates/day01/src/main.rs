use std::io::stdin;

fn main() {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let s = stdin()
        .lines()
        .map(|x| x.unwrap())
        .map(|line| {
            let mut digits = Vec::new();
            for i in 0..line.len() {
                let part = &line[i..];
                for (i, num) in nums.iter().enumerate() {
                    if part.starts_with(num) {
                        digits.push(i + 1);
                    }
                }
                for i in 0..=9 {
                    if part.starts_with(&format!("{i}")) {
                        digits.push(i);
                    }
                }
            }
            *digits.first().unwrap() * 10 + *digits.last().unwrap()
        })
        .sum::<usize>();
    println!("{s}");
}
