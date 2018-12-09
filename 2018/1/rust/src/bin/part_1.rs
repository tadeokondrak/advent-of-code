fn main() {
    println!(
        "{}",
        include_str!("../../../input")
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>()
    );
}
