fn main() {
    let input = include_str!("../input");
    let mut frequency: i32 = 0;
    for line in input.lines() {
        frequency += line.parse::<i32>().expect("Not an integer");
    }
    println!("Frequency: {}", frequency);
}
