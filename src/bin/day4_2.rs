use std::collections::HashMap;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone)]
struct Card {
    copies: Option<Vec<Card>>,
}

impl Card {
    fn add_card(&mut self, card: &Card) {
        if self.copies.is_none() {
            self.copies = Some(Vec::new());
        }
        self.copies.as_mut().unwrap().push(card.clone());
    }
    fn total_copies(&self) -> usize {
        match &self.copies {
            Some(cards) => 1 + cards.iter().map(|card| card.total_copies()).sum::<usize>(),
            None => 1,
        }
    }
}

fn day4_2(data: &Vec<String>) -> usize {
    let card_points = data
        .iter()
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|card| {
            let numbers = card[1].split('|').collect::<Vec<&str>>();
            let my_numbers = numbers[0]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let winning_numbers = numbers[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .filter(|n| my_numbers.contains(n))
                .collect::<Vec<u32>>();
            winning_numbers.len()
        })
        .collect::<Vec<usize>>();

    let mut cards: HashMap<usize, Card> = HashMap::new();

    for (i, c) in card_points.iter().enumerate().rev() {
        cards.insert(i, Card { copies: None });
        // if there are more than 1 copies to make
        if c > &0 {
            // println!("  adding {c} cards -> {:?}", [i + 1..i + c]);
            for idx in (i + 1)..=(i + c) {
                let card_to_copy = cards.get(&idx).unwrap().clone();
                // println!("    copying {} -> {}", card_to_copy.id, idx);
                cards.get_mut(&i).unwrap().add_card(&card_to_copy);
            }
        }
    }

    cards
        .iter()
        .map(|(_, card)| card.total_copies())
        .sum::<usize>()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day4_1.txt").unwrap();

    let sum = day4_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day4_2};

    #[test]
    fn day4_res() {
        let d = advent_of_code::Reader::read_file("./input/day4_1_test.txt").unwrap();
        let result = day4_2(&d);
        assert_eq!(result, 30);
    }

    #[test]
    fn day4_final() {
        let d = advent_of_code::Reader::read_file("./input/day4_1.txt").unwrap();
        let result = day4_2(&d);
        assert_eq!(result, 8736438);
    }
}
