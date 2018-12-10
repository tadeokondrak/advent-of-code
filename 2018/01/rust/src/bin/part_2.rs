use std::collections::HashSet;

fn main() {
    let mut frequency: i32 = 0;
    let mut seen = HashSet::new();
    for number in include_str!("../../../input")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
    {
        frequency += number;
        if seen.contains(&frequency) {
            println!("{}", frequency);
            break;
        }
        seen.insert(frequency);
    }
}
