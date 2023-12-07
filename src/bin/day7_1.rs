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
    let hand = "KK677".to_owned();
    let Ace = Card::A;
    let King = Card::K;
    let Queen = Card::Q;

    println!("{}", HandType::FiveOfAKind > HandType::FourOfAKind);

    let hand = Hand::from_string(hand.as_str());
    if hand.card_map.values().any(|v| *v == 2usize) {
        println!("TWO OF A K?IND: {:?}", hand.card_map);
    }
    println!(
        "cards: {:?}, {:?} TYPE: {:?}",
        hand.cards, hand.card_map, hand.hand_type
    );
    // let d = advent_of_code::Reader::read_file("./input/day7_1.txt").unwrap();
    //
    // let sum = day7_1(&d);
    // println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day7_1};

    #[test]
    fn day5_res() {
        let d = advent_of_code::Reader::read_file("./input/day7_1_test.txt").unwrap();
        let result = day7_1(&d);
        println!("result: {result}");
        assert_eq!(result, 35);
    }

    #[test]
    fn day5_final() {
        let d = advent_of_code::Reader::read_file("./input/day7_1.txt").unwrap();
        let result = day7_1(&d);
        assert_eq!(result, 484023871);
    }
}
