#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Number {
    left: usize,
    right: usize,
    row: usize,
    asterisk: (usize, usize),
    value: u32,
}

fn day3_2(data: &Vec<String>) -> u32 {
    let data_2d = data
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // find all numbers and filter for those that are adjacent to asterisks
    let mut nums = data
        .iter()
        .enumerate()
        .flat_map(|(i, l)| find_numbers(i, l))
        .filter_map(|n| has_star(n, &data_2d))
        .collect::<Vec<Number>>();

    // HashMap for asterisks and adjacent numbers
    let mut gears = HashMap::new();
    for n in &mut nums {
        gears.entry(n.asterisk).or_insert(Vec::new()).push(n.value);
    }

    // Filter the HashMap for values with exactly 2 in length
    let filtered_map: HashMap<_, _> = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(&k, v)| (k, v.clone())) // Clone or adjust as needed
        .collect();

    filtered_map
        .iter()
        .map(|(_, v)| v.iter().product::<u32>())
        .sum::<u32>()
}

fn find_numbers(i: usize, s: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    // Define a regex pattern to match numeric substrings
    let re = Regex::new(r"\d+").unwrap();

    // Iterate over regex matches and print start and end indices
    for mat in re.find_iter(s) {
        let start_index = mat.start();
        let end_index = mat.end();
        let value = s[start_index..end_index].parse::<u32>().unwrap();
        let n = Number {
            left: start_index,
            right: end_index,
            row: i,
            asterisk: (0, 0),
            value: value,
        };
        numbers.push(n);
    }
    numbers
}

// bit clunky. It would be better to first run over all chars and figure out if they are surrounded by symbols
fn has_star(n: Number, data_2d: &Vec<Vec<char>>) -> Option<Number> {
    let size = data_2d[0].len();
    let mut n = n.clone(); // make mutable clone of n
    for i in n.left..n.right {
        let idx = get_surrounding_indices(n.row, i, size);
        for p in idx {
            // println!("{}", data_2d[p.0][p.1]);
            match data_2d[p.0][p.1] {
                '*' => {
                    n.asterisk = p;
                    return Some(n);
                }
                _ => continue,
            };
        }
    }
    None
}

fn get_surrounding_indices(row: usize, col: usize, size: usize) -> Vec<(usize, usize)> {
    let mut surrounding_indices = Vec::new();

    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            // Check if the indices are within bounds
            if i >= 0 && i < size as isize && j >= 0 && j < size as isize {
                // Avoid adding the current element itself
                if !(i == row as isize && j == col as isize) {
                    surrounding_indices.push((i as usize, j as usize));
                }
            }
        }
    }

    surrounding_indices
}
fn main() {
    let d = advent_of_code::Reader::read_file("./input/day3_1.txt").unwrap();

    let sum = day3_2(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day3_2};

    #[test]
    fn day3_res() {
        let d = advent_of_code::Reader::read_file("./input/day3_1_test.txt").unwrap();
        let result = day3_2(&d);
        assert_eq!(result, 467835);
    }

    // #[test]
    // fn day3_final() {
    //     let d = advent_of_code::Reader::read_file("./input/day3_1.txt").unwrap();
    //     let result = day3_1(&d);
    //     assert_eq!(result, 531561);
    // }
}
