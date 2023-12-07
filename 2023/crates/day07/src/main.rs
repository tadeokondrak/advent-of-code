use std::{
    cmp::Ordering,
    io::{stdin, Read},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum JokerStatus {
    Disabled,
    Enabled,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve(&input, JokerStatus::Disabled));
    eprintln!("p2: {}", solve(&input, JokerStatus::Enabled));
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

fn score_hand(cards: [Card; 5], joker_status: JokerStatus) -> Hand {
    fn check_n_of_a_kind(cards: [Card; 5], n: usize, joker_status: JokerStatus) -> bool {
        cards.into_iter().any(|ref_card| {
            cards
                .iter()
                .copied()
                .filter(|&card| {
                    card == ref_card
                        || (joker_status == JokerStatus::Enabled && card == Card::CardJ)
                })
                .count()
                == n
        })
    }

    fn check_n_of_two_kinds(cards: [Card; 5], n: usize, joker_status: JokerStatus) -> bool {
        let joker_count = cards
            .iter()
            .copied()
            .filter(|&card| joker_status == JokerStatus::Enabled && card == Card::CardJ)
            .count();
        cards
            .iter()
            .copied()
            .filter(|&card_a| joker_status == JokerStatus::Disabled || card_a != Card::CardJ)
            .any(|card_a| {
                let card_a_count = cards.iter().copied().filter(|&card| card == card_a).count();
                cards
                    .iter()
                    .copied()
                    .filter(|&card_b| {
                        card_b != card_a
                            && (joker_status == JokerStatus::Disabled || card_b != Card::CardJ)
                    })
                    .any(|card_b| {
                        let card_b_count =
                            cards.iter().copied().filter(|&card| card == card_b).count();
                        card_a_count + card_b_count + joker_count == n
                    })
            })
    }

    if check_n_of_a_kind(cards, 5, joker_status) {
        return Hand::FiveOfAKind;
    }

    if check_n_of_a_kind(cards, 4, joker_status) {
        return Hand::FourOfAKind;
    }

    if check_n_of_two_kinds(cards, 5, joker_status) {
        return Hand::FullHouse;
    }

    if check_n_of_a_kind(cards, 3, joker_status) {
        return Hand::ThreeOfAKind;
    }

    if check_n_of_two_kinds(cards, 4, joker_status) {
        return Hand::TwoPair;
    }

    if check_n_of_a_kind(cards, 2, joker_status) {
        return Hand::OnePair;
    }

    Hand::HighCard
}

fn parse_hand(hand: &str) -> [Card; 5] {
    <[u8; 5]>::try_from(hand.as_bytes())
        .unwrap()
        .map(|c| match c {
            b'A' => Card::CardA,
            b'K' => Card::CardK,
            b'Q' => Card::CardQ,
            b'J' => Card::CardJ,
            b'T' => Card::CardT,
            b'9' => Card::Card9,
            b'8' => Card::Card8,
            b'7' => Card::Card7,
            b'6' => Card::Card6,
            b'5' => Card::Card5,
            b'4' => Card::Card4,
            b'3' => Card::Card3,
            b'2' => Card::Card2,
            _ => panic!(),
        })
}

fn solve(input: &str, joker_status: JokerStatus) -> u32 {
    let comparer: fn(&Card, &Card) -> Ordering = match joker_status {
        JokerStatus::Disabled => Card::cmp,
        JokerStatus::Enabled => |a, b| match (a, b) {
            (Card::CardJ, Card::CardJ) => Ordering::Equal,
            (_, Card::CardJ) => Ordering::Greater,
            (Card::CardJ, _) => Ordering::Less,
            (_, _) => a.cmp(&b),
        },
    };

    let mut hands_and_bids = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            (parse_hand(hand), bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    hands_and_bids.sort_by(|&(hand_a, _), &(hand_b, _)| {
        score_hand(hand_a, joker_status)
            .cmp(&score_hand(hand_b, joker_status))
            .then_with(|| {
                hand_a
                    .iter()
                    .copied()
                    .zip(hand_b.iter().copied())
                    .map(|(a, b)| comparer(&a, &b))
                    .filter(|ord| !ord.is_eq())
                    .next()
                    .unwrap()
            })
    });

    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn part_1() {
        fn check(hand: &str, result: Hand) {
            assert_eq!(score_hand(parse_hand(hand), JokerStatus::Disabled), result);
        }

        check("23456", Hand::HighCard);
        check("A23A4", Hand::OnePair);
        check("23432", Hand::TwoPair);
        check("TTT98", Hand::ThreeOfAKind);
        check("23332", Hand::FullHouse);
        check("AA8AA", Hand::FourOfAKind);
        check("AAAAA", Hand::FiveOfAKind);

        check("32T3K", Hand::OnePair);
        check("KK677", Hand::TwoPair);
        check("T55J5", Hand::ThreeOfAKind);
        check("KTJJT", Hand::TwoPair);
        check("QQQJA", Hand::ThreeOfAKind);

        assert_eq!(solve(INPUT, JokerStatus::Disabled), 6440);
    }

    #[test]
    fn part_2() {
        fn check(hand: &str, result: Hand) {
            assert_eq!(score_hand(parse_hand(hand), JokerStatus::Enabled), result);
        }

        check("32T3K", Hand::OnePair);
        check("KK677", Hand::TwoPair);
        check("T55J5", Hand::FourOfAKind);
        check("KTJJT", Hand::FourOfAKind);
        check("QQQJA", Hand::FourOfAKind);

        assert_eq!(solve(INPUT, JokerStatus::Enabled), 5905);
    }
}
