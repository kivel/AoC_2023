use std::collections::HashMap;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;
use advent_of_code::RingBuffer;

#[derive(Debug, Copy, Clone)]
enum Direction {
    L,
    R,
}

fn day8_2(data: &Vec<String>) -> usize {
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

    let mut current_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect::<Vec<&str>>();

    println!("NODES: {:?}", &current_nodes);

    let mut steps = 1;

    loop {
        let dir = ring_buffer.next();
        let tmp_nodes = current_nodes
            .iter()
            .map(|k| {
                let node = nodes.get(k);
                match dir {
                    Direction::L => node.unwrap().0,
                    Direction::R => node.unwrap().1,
                }
            })
            .collect::<Vec<&str>>();

        let abort = tmp_nodes.iter().all(|node| node.ends_with('Z'));
        if abort {
            break;
        }

        if steps % 100000 == 0 {
            println!("steps: {steps}");
        }
        current_nodes = tmp_nodes;
        steps += 1;
    }
    steps
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day8_1.txt").unwrap();
    let result = day8_2(&d);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day8_2};

    #[test]
    fn day8_res() {
        let d = advent_of_code::Reader::read_file("./input/day8_2_test.txt").unwrap();
        let result = day8_2(&d);
        println!("result 1: {result}");
        assert_eq!(result, 6);
    }

    #[test]
    fn day8_final() {
        let d = advent_of_code::Reader::read_file("./input/day8_1.txt").unwrap();
        let result = day8_2(&d);
        assert_eq!(result, 20093);
    }
}
