#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

use regex::Regex;

#[derive(Debug)]
struct Number {
    left: usize,
    right: usize,
    row: usize,
    value: u32,
}

fn day3_1(data: &Vec<String>) -> u32 {
    let data_2d = data
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    data.iter()
        .enumerate()
        .flat_map(|(i, l)| find_numbers(i, l))
        .filter(|n| has_symbol(&n, &data_2d))
        .map(|n| n.value)
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
            value: value,
        };
        numbers.push(n);
    }
    numbers
}

// bit clunky. It would be better to first run over all chars and figure out if they are surrounded by symbols
fn has_symbol(n: &Number, data_2d: &Vec<Vec<char>>) -> bool {
    let size = data_2d[0].len();
    for i in n.left..n.right {
        let idx = get_surrounding_indices(n.row, i, size);
        for p in idx {
            // println!("{}", data_2d[p.0][p.1]);
            match data_2d[p.0][p.1] {
                '.' => continue,
                c if c.is_ascii_digit() => continue,
                c if c.is_ascii_punctuation() => return true,
                _ => continue,
            };
        }
    }
    false
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

    let sum = day3_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day3_1};

    #[test]
    fn day3_res() {
        let d = advent_of_code::Reader::read_file("./input/day3_1_test.txt").unwrap();
        let result = day3_1(&d);
        assert_eq!(result, 4361);
    }

    #[test]
    fn day3_final() {
        let d = advent_of_code::Reader::read_file("./input/day3_1.txt").unwrap();
        let result = day3_1(&d);
        assert_eq!(result, 531561);
    }
}
