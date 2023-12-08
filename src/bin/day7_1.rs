use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Range;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    card_map: HashMap<Card, usize>,
    hand_type: HandType,
}

impl Hand {
    fn from_string(s: &str) -> Hand {
        let mut cards = Vec::new();
        let mut card_map = HashMap::new();
        for c in s.chars() {
            let card = Card::from_char(c);
            cards.push(card.clone());
            card_map
                .entry(card.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let hand_type = HandType::from_card_map(&card_map);
        Hand {
            cards,
            card_map,
            hand_type,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // in case the hand_type is the same, sort the cards
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return Some(self.cards[i].value().cmp(&other.cards[i].value()));
                }
            }
        }
        Some(self.hand_type.cmp(&other.hand_type))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // in case the hand_type is the same, sort the cards
        if self.hand_type == other.hand_type {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].value().cmp(&other.cards[i].value());
                }
            }
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    fn from_card_map(hm: &HashMap<Card, usize>) -> HandType {
        match hm.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => match hm.values().any(|v| *v == 2usize) {
                true => HandType::TwoPair,
                false => HandType::ThreeOfAKind,
            },
            2 => match hm.values().any(|v| *v == 3usize) {
                true => HandType::FullHouse,
                false => HandType::FourOfAKind,
            },
            1 => HandType::FiveOfAKind,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u8),
}
impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Number(value) => *value,
        }
    }

    fn from_char(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            _ => Card::Number(c as u8 - '0' as u8),
        }
    }
}
fn day7_1(data: &Vec<String>) -> usize {
    todo!()
}

fn main() {
    let hand1 = "KK677".to_owned();
    let hand2 = "KK676".to_owned();
    let Ace = Card::A;
    let King = Card::K;
    let Queen = Card::Q;

    println!("{}", HandType::FiveOfAKind > HandType::FourOfAKind);

    let handA = Hand::from_string(hand1.as_str());
    if handA.card_map.values().any(|v| *v == 2usize) {
        println!("TWO OF A K?IND: {:?}", handA.card_map);
    }
    println!(
        "cards: {:?}, map: {:?},  type: {:?}",
        handA.cards, handA.card_map, handA.hand_type
    );
    let handB = Hand::from_string(hand2.as_str());
    println!("hand1 == hand2 --> {}", handA == handB);
    println!("hand1 >= hand2 --> {}", handA == handB);
    println!("hand1 <= hand2 --> {}", handA == handB);
    let mut hands = vec![handB, handA];
    println!("before hands: {:?}", hands);
    hands.sort();
    println!("after hands: {:?}", hands);
    // let d = advent_of_code::Reader::read_file("./input/day7_1.txt").unwrap();
    //
    // let sum = day7_1(&d);
    // println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day7_1, Hand};

    #[test]
    fn day5_res() {
        // let d = advent_of_code::Reader::read_file("./input/day7_1_test.txt").unwrap();
        let hand1 = Hand::from_string("32T3K");
        let hand2 = Hand::from_string("T55J5");
        let hand3 = Hand::from_string("KK677");
        let hand4 = Hand::from_string("KTJJT");
        let hand5 = Hand::from_string("QQQJA");

        let mut hands = vec![hand1, hand2, hand3, hand4, hand5];

        for (index, hand) in hands.iter().enumerate() {
            println!("Hand {}: {:?}", index + 1, hand.cards);
        }

        println!("sorted");
        hands.sort();

        let expected = vec!["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"];
        for (index, hand) in hands.iter().enumerate() {
            println!(
                "Hand {}: expected {:?} -> {:?}",
                index + 1,
                expected[index],
                hand.cards
            );
        }

        // 32T3K -> KTJJT -> KK677 -> T55J5 --> QQQJA

        // let result = day7_1(&d);
        // println!("result: {result}");
        // assert_eq!(result, 35);
    }

    #[test]
    fn day5_final() {
        let d = advent_of_code::Reader::read_file("./input/day7_1.txt").unwrap();
        let result = day7_1(&d);
        assert_eq!(result, 484023871);
    }
}
