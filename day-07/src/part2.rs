use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../aoc-2023-inputs/day-07/input.txt");
    dbg!(part_2(input));
}

fn part_2(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line))
        .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.cmp(b));
    hands
        .into_iter()
        .map(|f| f.bid)
        .enumerate()
        .reduce(|prev, (index, hand)| (index, prev.1 + hand * u64::try_from(index + 1).unwrap()))
        .unwrap()
        .1
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    J = 0,
    T = 9,
    N9 = 8,
    N8 = 7,
    N7 = 6,
    N6 = 5,
    N5 = 4,
    N4 = 3,
    N3 = 2,
    N2 = 1,
}
impl Card {
    pub fn from_char(c: &char) -> Option<Card> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::N9),
            '8' => Some(Card::N8),
            '7' => Some(Card::N7),
            '6' => Some(Card::N6),
            '5' => Some(Card::N5),
            '4' => Some(Card::N4),
            '3' => Some(Card::N3),
            '2' => Some(Card::N2),
            _ => None,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u64,
}
#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}
impl Hand {
    pub fn from_str(s: &str) -> Hand {
        let split = s.split(" ").collect::<Vec<&str>>();
        let mut cards = Vec::new();
        for c in split[0].chars() {
            cards.push(Card::from_char(&c).unwrap());
        }
        Hand {
            cards,
            bid: split[1].parse::<u64>().unwrap(),
        }
    }
    pub fn sorted(&self) -> Hand {
        let mut new_hand = Hand {
            cards: self.cards.clone(),
            bid: self.bid,
        };
        new_hand.cards.sort();
        new_hand
    }
    pub fn hand_type(&self) -> HandType {
        let hand_sorted_og = self.sorted().cards;
        let mut hand_sorted_dedup = hand_sorted_og.clone();
        hand_sorted_dedup.dedup();
        hand_sorted_dedup = hand_sorted_dedup
            .into_iter()
            .filter(|card| card != &Card::J)
            .collect();

        let mut card_map: HashMap<Card, u32> = HashMap::new();
        for card in hand_sorted_og.iter() {
            let count = card_map.entry(card.clone()).or_insert(0);
            *count += 1;
        }

        if hand_sorted_dedup.len() == 1 || hand_sorted_dedup.len() == 0 {
            HandType::FiveOfAKind
        } else if hand_sorted_dedup.len() == 2 {
            let joker_count = *card_map.get(&Card::J).get_or_insert(&0);

            if *joker_count >= 2 {
                HandType::FourOfAKind
            } else if *joker_count == 1 {
                if card_map.values().any(|&x| x == 3) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            } else {
                if card_map.values().any(|&x| x == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
        } else if hand_sorted_dedup.len() == 3 {
            // Three of a kind or two pairs
            let joker_count = *card_map.get(&Card::J).get_or_insert(&0);

            if *joker_count >= 1 {
                HandType::ThreeOfAKind
            } else {
                if card_map.values().any(|&x| x == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPairs
                }
            }
        } else if hand_sorted_dedup.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    pub fn cmp(&self, b: &Hand) -> Ordering {
        let a_hand = self.hand_type();
        let b_hand = b.hand_type();

        if a_hand == b_hand {
            for (index, card) in self.cards.iter().enumerate() {
                if card > &b.cards[index] {
                    return Ordering::Greater;
                } else if card < &b.cards[index] {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else if a_hand > b_hand {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

#[cfg(test)]
mod tests_day07_02 {
    use super::*;

    #[test]
    fn test_parsing_input() {
        let input = "32T3K 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.cards.len(), 5);
        let hand_sorted = hand.sorted();
        assert_eq!(hand_sorted.cards[0], Card::N2);
        assert_eq!(hand_sorted.cards[1], Card::N3);
        assert_eq!(hand_sorted.cards[3], Card::T);
        assert_eq!(hand_sorted.cards[4], Card::K);
    }

    #[test]
    fn test_hand_type() {
        let input = "32T3K 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::OnePair);

        let input = "AA8AA 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);

        let input = "K33JK 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::FullHouse);

        let input = "K8J4J 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);

        let input = "KTJ44 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);

        let input = "A2345 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::HighCard);

        let input = "KJJKK 0";
        let hand = Hand::from_str(input);
        assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../aoc-2023-inputs/day-07/test.txt");
        assert_eq!(part_2(input), 5905);
    }
}

