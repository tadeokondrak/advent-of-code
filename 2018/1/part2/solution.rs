use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let mut frequency: i32 = 0;
    let mut seen = HashSet::new();
    for line in input.lines().cycle() {
        let number: i32 = line.parse::<i32>().expect("Not an integer");
        frequency += number;
        if seen.contains(&frequency) {
            println!("Frequency: {}", frequency);
            break;
        }
        seen.insert(frequency);
    }
}
