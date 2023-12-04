#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day4_1(data: &Vec<String>) -> u32 {
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
            winning_numbers.len() as u32
        })
        .collect::<Vec<u32>>();

    card_points
        .iter()
        .filter(|v| *v > &0)
        .map(|v| u32::pow(2, v - 1))
        .sum::<u32>()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day4_1.txt").unwrap();

    let sum = day4_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day4_1};

    #[test]
    fn day3_res() {
        let d = advent_of_code::Reader::read_file("./input/day4_1_test.txt").unwrap();
        let result = day4_1(&d);
        assert_eq!(result, 13);
    }

    #[test]
    fn day3_final() {
        let d = advent_of_code::Reader::read_file("./input/day4_1.txt").unwrap();
        let result = day4_1(&d);
        assert_eq!(result, 24542);
    }
}
