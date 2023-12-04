use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (winning, ours) = line.split_once(" | ").unwrap();
            let winning = winning
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let mut ours = ours
                .split(" ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            ours.sort();
            let matching_count = winning
                .iter()
                .filter(|winning| ours.binary_search(&winning).is_ok())
                .count() as u32;
            if matching_count > 0 {
                2u32.pow(matching_count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn solve_p2(input: &str) -> u32 {
    let mut cards = Vec::new();
    let mut ourses = Vec::new();
    for line in input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
    {
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, ours) = line.split_once(" | ").unwrap();
        let winning = winning
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut ours = ours
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        ours.sort();
        cards.push(winning);
        ourses.push(ours);
    }
    let mut count = 0;
    let mut queue = Vec::new();
    queue.extend(0..cards.len());
    while let Some(card_index) = queue.pop() {
        let card = &cards[card_index];
        let ours = &ourses[card_index];
        for (i, _) in card
            .iter()
            .filter(|winning| ours.binary_search(&winning).is_ok())
            .enumerate()
        {
            queue.push(card_index + 1 + i);
        }
        count += 1;
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            ),
            13
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            ),
            30
        );
    }
}
