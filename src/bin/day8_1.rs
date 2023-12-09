use regex::Regex;
use std::collections::HashMap;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;
use advent_of_code::RingBuffer;

#[derive(Debug, Copy, Clone)]
enum Direction {
    L,
    R,
}

fn day8_1(data: &Vec<String>) -> usize {
    let directions = data
        .first()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::R,
            'L' => Direction::L,
            _ => unreachable!(),
        })
        .collect::<Vec<Direction>>();
    println!("{:?}", &directions);
    let mut ring_buffer = RingBuffer::new(directions);

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let mut key = "";
    let mut left = "";
    let mut right = "";
    if let Some(captures) = re.captures(data[2].as_str()) {
        key = captures.get(1).unwrap().as_str();
        left = captures.get(2).unwrap().as_str();
        right = captures.get(3).unwrap().as_str();
    }

    // let mut nodes: HashMap<&str, Node> = HashMap::new();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    data.iter().skip(2).for_each(|s| {
        let mut split = s.split(" = ");
        let key = split.next().unwrap();
        let mut value = split
            .next()
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ");
        nodes.insert(key, (value.next().unwrap(), value.next().unwrap()));
    });

    let mut steps = 0;

    let mut lookup = nodes.get(&key);
    println!("first node: {:?}", lookup);

    while key != "ZZZ" {
        key = match ring_buffer.next() {
            Direction::L => lookup.unwrap().0,
            Direction::R => lookup.unwrap().1,
        };
        steps += 1;
        lookup = nodes.get(&key);

        if steps > 50000000 {
            panic!("the answer is too high");
        }
    }
    steps
}
// 25000000

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day8_1.txt").unwrap();
    let result = day8_1(&d);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day8_1};

    #[test]
    fn day8_res() {
        let d = advent_of_code::Reader::read_file("./input/day8_1_test.txt").unwrap();
        let result = day8_1(&d);
        println!("result 1: {result}");
        assert_eq!(result, 2);
        let d = advent_of_code::Reader::read_file("./input/day8_1_test_2.txt").unwrap();
        let result = day8_1(&d);
        println!("result 2: {result}");
        assert_eq!(result, 6);
    }

    #[test]
    fn day8_final() {
        let d = advent_of_code::Reader::read_file("./input/day8_1.txt").unwrap();
        let result = day8_1(&d);
        assert_eq!(result, 251121738);
    }
}
