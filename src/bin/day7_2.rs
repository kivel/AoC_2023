use std::collections::HashMap;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    card_map: HashMap<Card, usize>,
    hand_type: HandType,
    bet: usize,
}

impl Hand {
    fn from_string(s: &str) -> Hand {
        let mut input = s.split_whitespace();
        let s = input.next().unwrap();
        let bet = input.next().unwrap().parse::<usize>().unwrap();

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
            bet,
        }
    }

    fn boost(&mut self) {
        // can't improve
        if !self.cards.contains(&Card::J) || self.hand_type == HandType::FiveOfAKind {
            return;
        }
        // keep going, as there is at least one joker
        let n = self.card_map.get(&Card::J).unwrap();
        if *n == 1 {
            match self.hand_type {
                HandType::HighCard => self.hand_type = HandType::OnePair,
                HandType::OnePair => self.hand_type = HandType::ThreeOfAKind,
                HandType::ThreeOfAKind => self.hand_type = HandType::FourOfAKind,
                HandType::FourOfAKind => self.hand_type = HandType::FiveOfAKind,
                HandType::TwoPair => self.hand_type = HandType::ThreeOfAKind,
                _ => {}
            }
        }
        if *n == 2 {
            match self.hand_type {
                // Can't be HighCard, if we have 2 jokers
                // HandType::HighCard => self.hand_type = HandType::ThreeOfAKind,
                HandType::OnePair => self.hand_type = HandType::ThreeOfAKind,
                // ThreeOfAKind is impossible with OnePair<J>, that would be FullHouse
                // HandType::ThreeOfAKind => self.hand_type = HandType::FourOfAKind,
                // Impossible to have FourOfAKind with OnePair<J>, that would be 6 cards
                // HandType::FourOfAKind => self.hand_type = HandType::FiveOfAKind,
                HandType::TwoPair => self.hand_type = HandType::FourOfAKind,
                HandType::FullHouse => self.hand_type = HandType::FiveOfAKind,
                _ => {}
            }
        }
        if *n == 3 {
            match self.hand_type {
                // can't be FullHouse, so we boost to FourOfAKind
                HandType::ThreeOfAKind => self.hand_type = HandType::FourOfAKind,
                HandType::FullHouse => self.hand_type = HandType::FiveOfAKind,
                _ => {}
            }
        }
        if *n == 4 {
            self.hand_type = HandType::FiveOfAKind
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
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
            Card::J => 1,
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
fn day7_2(data: &Vec<String>) -> usize {
    let mut hands: Vec<Hand> = data.iter().map(|h| Hand::from_string(h)).collect();

    // boost hands based on jokers
    hands.iter_mut().for_each(|h| h.boost());

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bet)
        .sum::<usize>()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day7_1.txt").unwrap();
    let result = day7_2(&d);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day7_2, Hand};

    #[test]
    fn day5_res() {
        let hand1 = Hand::from_string("32T3K 765");
        let hand2 = Hand::from_string("T55J5 684");
        let hand3 = Hand::from_string("KK677 28");
        let hand4 = Hand::from_string("KTJJT 220");
        let hand5 = Hand::from_string("QQQJA 483");

        let mut hands = vec![hand1, hand2, hand3, hand4, hand5];

        hands.iter_mut().for_each(|h| h.boost());

        hands.sort();

        let mut result = 0;
        for (index, hand) in hands.iter().enumerate() {
            result += hand.bet * (index + 1);
            println!(
                "Hand {}: bet: {}, hand: {:?}",
                index + 1,
                hand.bet,
                hand.hand_type
            );
        }

        println!("result: {result}");
        assert_eq!(result, 5905);
    }

    #[test]
    fn day5_test_file() {
        let d = advent_of_code::Reader::read_file("./input/day7_1_test.txt").unwrap();
        let result = day7_2(&d);
        assert_eq!(result, 5905);
    }
    #[test]
    fn day5_final() {
        let d = advent_of_code::Reader::read_file("./input/day7_1.txt").unwrap();
        let result = day7_2(&d);
        // 251337575 is too low
        assert!(result > 251337575);
    }
}
