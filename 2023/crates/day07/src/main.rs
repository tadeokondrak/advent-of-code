#![feature(iter_order_by)]
use std::{
    cmp::Ordering,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve(&input, score_hand_p1, Card::cmp_p1));
    eprintln!("p2: {}", solve(&input, score_hand_p2, Card::cmp_p2));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardJ,
    CardQ,
    CardK,
    CardA,
}

const ALL_CARDS: [Card; 13] = [
    Card::Card2,
    Card::Card3,
    Card::Card4,
    Card::Card5,
    Card::Card6,
    Card::Card7,
    Card::Card8,
    Card::Card9,
    Card::CardT,
    Card::CardJ,
    Card::CardQ,
    Card::CardK,
    Card::CardA,
];

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::CardA),
            'K' => Some(Card::CardK),
            'Q' => Some(Card::CardQ),
            'J' => Some(Card::CardJ),
            'T' => Some(Card::CardT),
            '9' => Some(Card::Card9),
            '8' => Some(Card::Card8),
            '7' => Some(Card::Card7),
            '6' => Some(Card::Card6),
            '5' => Some(Card::Card5),
            '4' => Some(Card::Card4),
            '3' => Some(Card::Card3),
            '2' => Some(Card::Card2),
            _ => None,
        }
    }

    fn cmp_p1(&self, other: &Card) -> Ordering {
        self.cmp(other)
    }

    fn cmp_p2(&self, other: &Card) -> Ordering {
        match (self, other) {
            (Card::CardJ, Card::CardJ) => Ordering::Equal,
            (_, Card::CardJ) => Ordering::Greater,
            (Card::CardJ, _) => Ordering::Less,
            (_, _) => self.cmp(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn score_hand_p1(cards: [Card; 5]) -> Hand {
    let counts = (0..=5)
        .map(|count| {
            ALL_CARDS
                .iter()
                .filter(|&ref_card| {
                    cards
                        .iter()
                        .copied()
                        .filter(|card| card == ref_card)
                        .count()
                        == count
                })
                .count()
        })
        .collect::<Vec<_>>();


    if counts[5] > 0 {
        return Hand::FiveOfAKind;
    }

    if counts[4] > 0 {
        return Hand::FourOfAKind;
    }

    if counts[3] == 1 && counts[2] == 1 {
        return Hand::FullHouse;
    }

    if counts[3] > 0 {
        return Hand::ThreeOfAKind;
    }

    if counts[2] > 1 {
        return Hand::TwoPair;
    }

    if counts[2] > 0 {
        return Hand::OnePair;
    }

    Hand::HighCard
}

fn score_hand_p2(cards: [Card; 5]) -> Hand {
    if let Some(i) = cards.iter().position(|&card| card == Card::CardJ) {
        let mut copy = cards;

        return ALL_CARDS
            .into_iter()
            .filter(|&card| card != Card::CardJ)
            .map(|card| {
                copy[i] = card;
                score_hand_p2(copy)
            })
            .max()
            .unwrap();
    }

    score_hand_p1(cards)
}

fn parse_hand(hand: &str) -> [Card; 5] {
    hand.chars()
        .map(|c| Card::from_char(c).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn solve(
    input: &str,
    scorer: fn([Card; 5]) -> Hand,
    comparer: fn(&Card, &Card) -> Ordering,
) -> i32 {
    let mut hands_and_bids = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            (parse_hand(hand), bid.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();
    hands_and_bids.sort_by(|&(hand_a, _), &(hand_b, _)| {
        scorer(hand_a)
            .cmp(&scorer(hand_b))
            .then_with(|| hand_a.iter().cmp_by(hand_b.iter(), comparer))
    });
    hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as i32 + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(score_hand_p1(parse_hand("23456")), Hand::HighCard);
        assert_eq!(score_hand_p1(parse_hand("A23A4")), Hand::OnePair);
        assert_eq!(score_hand_p1(parse_hand("23432")), Hand::TwoPair);
        assert_eq!(score_hand_p1(parse_hand("TTT98")), Hand::ThreeOfAKind);
        assert_eq!(score_hand_p1(parse_hand("23332")), Hand::FullHouse);
        assert_eq!(score_hand_p1(parse_hand("AA8AA")), Hand::FourOfAKind);
        assert_eq!(score_hand_p1(parse_hand("AAAAA")), Hand::FiveOfAKind);

        assert_eq!(
            solve(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
                score_hand_p1,
                Card::cmp_p1
            ),
            6440
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(score_hand_p2(parse_hand("32T3K")), Hand::OnePair);
        assert_eq!(score_hand_p2(parse_hand("KK677")), Hand::TwoPair);
        assert_eq!(score_hand_p2(parse_hand("T55J5")), Hand::FourOfAKind);
        assert_eq!(score_hand_p2(parse_hand("KTJJT")), Hand::FourOfAKind);
        assert_eq!(score_hand_p2(parse_hand("QQQJA")), Hand::FourOfAKind);

        assert_eq!(
            solve(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
                score_hand_p2,
                Card::cmp_p2
            ),
            5905
        );
    }
}
